use std::collections::HashMap;

use crate::dataset::Dataset;
use crate::errors::{CatalogError, ValidationError};
use crate::validation::validate_name;


#[derive(Debug, Clone, PartialEq, Eq)]
struct Catalog {
    name: String,
    datasets: HashMap<String, Dataset>
}

impl Catalog {
    fn new(name: String) -> Result<Self, ValidationError>{
        validate_name(&name)?;
        Ok(Catalog {
            name,
            datasets: HashMap::new(),
        })
    }

    fn register_dataset(&mut self, new_dataset: Dataset) -> Result<(), CatalogError> {
        let new_dataset_name: &str = new_dataset.get_name();
        if self.datasets.contains_key(new_dataset_name) {
            return Err(CatalogError::DuplicateDatasetName);
        }
        self.datasets.insert(new_dataset_name.to_string(), new_dataset);
        Ok(())
    }

    fn get_dataset(&self, dataset_name: &str) -> Result<&Dataset, CatalogError> {
        match self.datasets.get(dataset_name) {
            Some(value) => Ok(value),
            None => Err(CatalogError::DatasetNotFound(dataset_name.to_string()))
        }
    }

    fn list_datasets(&self) -> impl Iterator<Item = &Dataset> {
        self.datasets.values()
    }

    fn remove_dataset(&mut self, dataset_name: &str) -> Result<Dataset, CatalogError> {
        match self.datasets.remove(dataset_name) {
            Some(value) => Ok(value),
            None => Err(CatalogError::DatasetNotFound(dataset_name.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {

    use std::{collections::HashSet, path::PathBuf};
    use crate::dataset::DataFormat;

use super::*;

    #[test]
    fn catalog_creation() {
        let catalog: Catalog = Catalog::new("test-catalog".to_string()).unwrap();
        assert_eq!(catalog.name, "test-catalog".to_string());
        assert_eq!(catalog.datasets, HashMap::new());
    }

    #[test]
    fn register_get_remove_dataset() {
        let mut catalog: Catalog = Catalog::new("test-catalog".to_string()).unwrap();
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.csv");
        let dataset = Dataset::new("test_dataset".to_string(), file_path.display().to_string()).unwrap();

        catalog.register_dataset(dataset).unwrap();
        let registered_dataset = catalog.get_dataset(&"test_dataset").unwrap();

        assert_eq!(registered_dataset.get_name(), "test_dataset");
        assert_eq!(registered_dataset.get_path(), &file_path);
        assert_eq!(registered_dataset.get_format(), &DataFormat::Csv);

        catalog.remove_dataset(&"test_dataset").unwrap();

        assert_eq!(
            catalog.get_dataset(&"test_dataset").unwrap_err(), 
            CatalogError::DatasetNotFound("test_dataset".to_string())
        )
    }

    #[test]
    fn list_datasets() {
        let mut catalog: Catalog = Catalog::new("test-catalog".to_string()).unwrap();
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.csv");
        let dataset_a = Dataset::new("test_dataset_a".to_string(), file_path.display().to_string()).unwrap();
        let dataset_b = Dataset::new("test_dataset_b".to_string(), file_path.display().to_string()).unwrap();

        catalog.register_dataset(dataset_a).unwrap();
        catalog.register_dataset(dataset_b).unwrap();
        let it = catalog.list_datasets();
        
        let expected: HashSet<&str> = HashSet::from(["test_dataset_a", "test_dataset_b"]);
        let actual: HashSet<&str> = catalog.list_datasets().map(|dataset| dataset.get_name()).collect();

        assert_eq!(it.count(), 2);
        assert_eq!(actual, expected);

    }

    #[test]
    fn register_dataset_key_already_exists() {
        let mut catalog: Catalog = Catalog::new("test-catalog".to_string()).unwrap();
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.csv");
        let dataset_a = Dataset::new("test_dataset_a".to_string(), file_path.display().to_string()).unwrap();
        let dataset_b = Dataset::new("test_dataset_a".to_string(), file_path.display().to_string()).unwrap();

        catalog.register_dataset(dataset_a).unwrap();

        assert_eq!(catalog.register_dataset(dataset_b).unwrap_err(), CatalogError::DuplicateDatasetName)
    }

    #[test]
    fn get_unknown_dataset() {
        let catalog: Catalog = Catalog::new("test-catalog".to_string()).unwrap();
        let unknown_dataset_name: String = String::from("");

        assert_eq!(catalog.get_dataset(&unknown_dataset_name).unwrap_err(), CatalogError::DatasetNotFound(unknown_dataset_name))
    }

    #[test]
    fn remove_unknown_dataset() {
        let mut catalog: Catalog = Catalog::new("test-catalog".to_string()).unwrap();
        let unknown_dataset_name: String = String::from("");

        assert_eq!(catalog.remove_dataset(&unknown_dataset_name).unwrap_err(), CatalogError::DatasetNotFound(unknown_dataset_name))
    }

}