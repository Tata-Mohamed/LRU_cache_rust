    #[cfg(test)]
    mod tests {
        use crate::{cache_lru::LruCache, cache_trait::CacheTrait};
        /// Teste les opérations basiques du cache LRU
        ///
        /// Ce test vérifie la bonne gestion de l'ajout et de la récupération d'éléments,
        /// ainsi que le respect de la capacité du cache
        #[test]
        fn test_lru_cache() {
            let mut cache: LruCache<String> = LruCache::new(3);
            cache.put("A", String::from("value_a"));
            cache.put("B", String::from("value_b"));
            cache.put("C", String::from("value_c"));
            cache.put("D", String::from("value_d"));
            // Premier élément moins récemment utilisé et dernier le plus récent
            // Cache == [value_b, value_c, value_d]

            let my_value = cache.get("A");
            assert_eq!(my_value, None); // A a été évincé

            let my_value = cache.get("D");
            assert_eq!(my_value, Some(&String::from("value_d")));
            // Cache == [value_b, value_c, value_d]

            let my_value = cache.get("B");
            assert_eq!(my_value, Some(&String::from("value_b")));
            // Cache == [value_c, value_d, value_b]

            let my_value = cache.get("C");
            assert_eq!(my_value, Some(&String::from("value_c")));
            // Cache == [value_d, value_b, value_c]

            let my_value = cache.get("X");
            assert_eq!(my_value, None);
            // Cache == [value_d, value_b, value_c]

            cache.put("A", String::from("value_a"));
            // Cache == [valueb, value_c, value_a]

            cache.put("X", String::from("value_x"));
            // Cache == [value_c, value_a, value_x]

            let my_value = cache.get("B");
            assert_eq!(my_value, None);
            // Cache == [value_c, value_a, value_x]

            let my_value = cache.get("D");
            assert_eq!(my_value, None);
            // Cache == [value_c, value_a, value_x]
        }

        #[test]
        fn test_put() {
            let mut cache: LruCache<String> = LruCache::new(3);

            cache.put("A", String::from("value_a"));

            // Vérifier que l'élément a été correctement inséré
            let my_value = cache.get("A");
            assert_eq!(my_value, Some(&String::from("value_a")));
        }

        // #[test]
        // fn test_get() {
        //     let mut cache: LruCache<String> = LruCache::new(3); // Taille de 3
        
        //     cache.put("A", String::from("value_a"));
        //     cache.put("B", String::from("value_b"));
        
        //     let my_value_a = cache.get("A");
        //     let my_value_b = cache.get("B");
        
        //     assert_eq!(my_value_a, Some(&String::from("value_a")));
        //     assert_eq!(my_value_b, Some(&String::from("value_b")));
        // }
        
        

        #[test]
        fn test_size() {
            let mut cache: LruCache<String> = LruCache::new(3);

            cache.put("A", String::from("value_a"));
            cache.put("B", String::from("value_b"));
            cache.put("C", String::from("value_c"));

            // Vérifier la taille du cache
            assert_eq!(cache.size(), 3);

            cache.put("D", String::from("value_d"));

            // La taille reste 3
            assert_eq!(cache.size(), 3);
        }
    }
