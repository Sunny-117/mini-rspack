#[cfg(test)]
mod module_tests {
    use mini_rspack::module::{Module, parse_module};
    use std::path::Path;

    #[test]
    fn test_module_creation() {
        let module = Module {
            id: "./src/test.js".to_string(),
            source: "console.log('test');".to_string(),
            dependencies: vec!["./dep1.js".to_string(), "./dep2.js".to_string()],
        };
        
        assert_eq!(module.id, "./src/test.js");
        assert_eq!(module.source, "console.log('test');");
        assert_eq!(module.dependencies.len(), 2);
        assert_eq!(module.dependencies[0], "./dep1.js");
        assert_eq!(module.dependencies[1], "./dep2.js");
    }

    #[test]
    fn test_parse_module_commonjs() {
        let source = r#"
            const dep1 = require('./dep1');
            const dep2 = require('./dep2');
            module.exports = { test: true };
        "#;
        
        let module_path = Path::new("test.js");
        let module = parse_module(source, module_path).unwrap();
        
        assert_eq!(module.dependencies.len(), 2);
        assert!(module.dependencies.contains(&"./dep1".to_string()));
        assert!(module.dependencies.contains(&"./dep2".to_string()));
    }

    #[test]
    fn test_parse_module_esm() {
        let source = r#"
            import dep1 from './dep1';
            import { named } from './dep2';
            export default function() { return 'test'; }
        "#;
        
        let module_path = Path::new("test.js");
        let module = parse_module(source, module_path).unwrap();
        
        assert_eq!(module.dependencies.len(), 2);
        assert!(module.dependencies.contains(&"./dep1".to_string()));
        assert!(module.dependencies.contains(&"./dep2".to_string()));
    }

    #[test]
    fn test_parse_module_dynamic_import() {
        let source = r#"
            import('./dep1').then(module => console.log(module));
            const loadModule = () => import('./dep2');
        "#;
        
        let module_path = Path::new("test.js");
        let module = parse_module(source, module_path).unwrap();
        
        assert_eq!(module.dependencies.len(), 2);
        assert!(module.dependencies.contains(&"./dep1".to_string()));
        assert!(module.dependencies.contains(&"./dep2".to_string()));
    }

    #[test]
    fn test_parse_module_mixed() {
        let source = r#"
            import dep1 from './dep1';
            const dep2 = require('./dep2');
            import('./dep3').then(module => console.log(module));
        "#;
        
        let module_path = Path::new("test.js");
        let module = parse_module(source, module_path).unwrap();
        
        assert_eq!(module.dependencies.len(), 3);
        assert!(module.dependencies.contains(&"./dep1".to_string()));
        assert!(module.dependencies.contains(&"./dep2".to_string()));
        assert!(module.dependencies.contains(&"./dep3".to_string()));
    }
}
