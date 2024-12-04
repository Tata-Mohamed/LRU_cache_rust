use std::collections::HashMap;
use crate::cache_trait::CacheTrait;

/// Ce cache conserve les éléments récemment utilisés et supprime automatiquement
/// les plus anciens lorsque la capacité maximale est atteinte
pub struct LruCache<V> {
    capacity: usize,
    map: HashMap<String, V>,
    order: Vec<String>,
}

impl<V> LruCache<V> {
    /// Crée une nouvelle instance de `LruCache` avec une capacité spécifiée.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Le nombre maximal d'éléments que le cache peut contenir.
    pub fn new(capacity: usize) -> Self {
        LruCache {
            capacity,
            map: HashMap::with_capacity(capacity),
            order: Vec::with_capacity(capacity),
        }
    }
}

impl<V> CacheTrait<V> for LruCache<V> {
    /// Clé-valeur dans le cache
    /// Si la clé existe déjà, sa valeur est mis à jour et sa position dans l'ordre est supprimée
    /// Si le cache est plein l'élément le plus ancien est supprimé
    ///
    /// # Arguments
    ///
    /// * `key` - La clé de l'élément à insérer.
    /// * `value` - La valeur associée à la clé.
    fn put(&mut self, key: &str, value: V) {
        let key = key.to_string(); // Converti le str en String pour le HashMap

        // Supprime l'élément si le cache le contient
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        }

        // Si le cache est plein on supprime le plus ancien
        if self.map.len() >= self.capacity {
            if let Some(oldest_key) = self.order.first().cloned() {
                self.map.remove(&oldest_key);
                self.order.remove(0);
            }
        }

        self.map.insert(key.clone(), value);

        // Ajoute la clé à la fin
        self.order.push(key);
    }

    /// Récupère la valeur associée à une clé, si elle existe dans le cache
    /// Cette opération met à jour la position de la nouvelle clé 
    ///
    /// # Arguments
    ///
    /// * `key` - La clé de l'élément à récupérer.
    ///
    /// # Retourne
    /// Une option qui contient une référence à la valeur qui est donnée à la clé, ou `None` si la clé n'eciste pas
    fn get(&mut self, key: &str) -> Option<&V> {
        let key = key.to_string();

        // Vérifie si le HashMap contient la clé
        if self.map.contains_key(&key) {
            // Met à jour l'ordre
            self.order.retain(|k| k != &key);
            self.order.push(key.clone());

            // Retourne la valeur de la clé
            return self.map.get(&key);
        }
        None
    }

    /// # Retourne
    /// La taille actuelle du cache
    fn size(&self) -> usize {
        self.map.len()
    }
}
