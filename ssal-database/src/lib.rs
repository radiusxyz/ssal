use std::{any, fmt::Debug, ops, path::Path, sync::Arc};

use ssal_core::{
    bincode,
    error::{Error, WrapError},
    rocksdb::{Options, Transaction, TransactionDB, TransactionDBOptions},
    serde::{de::DeserializeOwned, ser::Serialize},
};

pub struct Database {
    client: Arc<TransactionDB>,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
        }
    }
}

impl Database {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut db_options = Options::default();
        db_options.create_if_missing(true);
        let tx_db_options = TransactionDBOptions::default();

        let client = TransactionDB::open(&db_options, &tx_db_options, &path).wrap(format!(
            "Failed to open the database at {:?}",
            path.as_ref(),
        ))?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub fn get<K, V>(&self, key: &K) -> Result<V, Error>
    where
        K: Debug + Serialize,
        V: Debug + DeserializeOwned,
    {
        let key_vec =
            bincode::serialize(key).wrap(format!("Failed to serialize the key: {:?}", key))?;

        let value_slice = self
            .client
            .get_pinned(key_vec)
            .wrap(format!("Failed to get the key: {:?}", key))?
            .wrap(format!("key: {:?}", key))?;

        let value: V = bincode::deserialize(value_slice.as_ref()).wrap(format!(
            "Failed to deserialize the value into type: {:?}",
            any::type_name::<V>(),
        ))?;

        Ok(value)
    }

    pub fn get_mut<K, V>(&self, key: &K) -> Result<Lock<V>, Error>
    where
        K: Debug + Serialize,
        V: Debug + DeserializeOwned + Serialize,
    {
        let key_vec =
            bincode::serialize(key).wrap(format!("Failed to serialize the key: {:?}", key))?;

        let transaction = self.client.transaction();
        let value_slice = transaction
            .get_for_update(&key_vec, true)
            .wrap(format!("Failed to get a lock for the key: {:?}", key))?
            .wrap(format!("key: {:?}", key))?;

        let value: V = bincode::deserialize(value_slice.as_ref()).wrap(format!(
            "Failed to deserialize the value into type: {:?}",
            any::type_name::<V>(),
        ))?;
        let locked_value = Lock::new(Some(transaction), key_vec, value);
        Ok(locked_value)
    }

    pub fn put<K, V>(&self, key: &K, value: &V) -> Result<(), Error>
    where
        K: Debug + Serialize,
        V: Debug + Serialize,
    {
        let key_vec =
            bincode::serialize(key).wrap(format!("Failed to serialize the key: {:?}", key))?;

        let value_vec =
            bincode::serialize(value).wrap(format!("Failed to serialize the value: {:?}", key))?;

        let transaction = self.client.transaction();
        transaction.put(key_vec, value_vec).wrap(format!(
            "Failed to put the value: {:?} for the key: {:?}",
            key, value,
        ))?;
        transaction.commit().wrap(format!(
            "Failed to commit put transaction for the key: {:?}",
            key,
        ))?;
        Ok(())
    }

    pub fn delete<K>(&self, key: &K) -> Result<(), Error>
    where
        K: Debug + Serialize,
    {
        let key_vec =
            bincode::serialize(key).wrap(format!("Failed to serialize the key: {:?}", key))?;

        let transaction = self.client.transaction();
        transaction
            .delete(key_vec)
            .wrap(format!("Failed to delete the key: {:?}", key))?;
        transaction.commit().wrap(format!(
            "Failed to commit delete transaction for the key: {:?}",
            key,
        ))?;
        Ok(())
    }
}

pub struct Lock<'db, V> {
    transaction: Option<Transaction<'db, TransactionDB>>,
    key_vec: Vec<u8>,
    value: V,
}

impl<'db, V> ops::Deref for Lock<'db, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'db, V> ops::DerefMut for Lock<'db, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<'db, V> Lock<'db, V>
where
    V: Debug + Serialize,
{
    fn new(
        transaction: Option<Transaction<'db, TransactionDB>>,
        key_vec: Vec<u8>,
        value: V,
    ) -> Self {
        Self {
            transaction,
            key_vec,
            value,
        }
    }

    pub fn commit(mut self) -> Result<(), Error> {
        if let Some(transaction) = self.transaction.take() {
            let value = bincode::serialize(&self.value)
                .wrap(format!("Failed to serialize the value: {:?}", &self.value))?;

            transaction
                .put(self.key_vec, value)
                .wrap(format!("Failed to put the value: {:?}", &self.value))?;
            transaction.commit().wrap(format!(
                "Failed to commit the transaction for the value: {:?}",
                &self.value,
            ))?;
        }
        Ok(())
    }
}
