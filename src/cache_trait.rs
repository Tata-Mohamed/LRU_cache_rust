/// Un trait qui définit les opérations d'un cache
/// Les implémentations de ce trait doivent gérer les opérations suivantes :
/// - Ajout d'éléments (`put`)
/// - Récupération d'éléments (`get`)
/// - Taille actuelle du cache (`size`)
pub trait CacheTrait<V> {
    /// Ajoute clé-valeur dans le cache
    ///
    /// # Arguments
    ///
    /// * `key` - La clé de l'élément à insérer
    /// * `value` - La valeur associée à la clé
    fn put(&mut self, key: &str, value: V);

    /// Récupère une valeur en fonction de la clé
    ///
    /// # Arguments
    ///
    /// * `key` - La clé de l'élément à récupérer
    ///
    /// # Retourne
    /// Une option qui contient une référence à la clé, ou `None` si la clé n'existe pas
    fn get(&mut self, key: &str) -> Option<&V>;

    /// # Retourne
    /// La taille actuelle du cache
    fn size(&self) -> usize;
}
