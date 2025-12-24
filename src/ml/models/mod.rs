use ndarray::{Array2, Array1};
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::ensemble::random_forest_regressor::RandomForestRegressor;
use smartcore::tree::decision_tree_regressor::DecisionTreeRegressorParameters;

pub struct PricePredictor {
    model: Option<RandomForestRegressor>,
}

impl PricePredictor {
    pub fn new() -> Self {
        Self { model: None }
    }
    
    pub fn train(&mut self, features: &Array2<f64>, targets: &Array1<f64>) -> Result<(), Box<dyn std::error::Error>> {
        // Convert ndarray to DenseMatrix
        let x = DenseMatrix::from_array(features.shape()[0], features.shape()[1], features.as_slice().unwrap());
        let y = targets.to_vec();
        
        let parameters = DecisionTreeRegressorParameters::default();
        let model = RandomForestRegressor::fit(&x, &y, Default::default())
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        
        self.model = Some(model);
        Ok(())
    }
    
    pub fn predict(&self, features: &Array2<f64>) -> Result<Array1<f64>, Box<dyn std::error::Error>> {
        if let Some(model) = &self.model {
            let x = DenseMatrix::from_array(features.shape()[0], features.shape()[1], features.as_slice().unwrap());
            let predictions = model.predict(&x)
                .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
            
            Ok(Array1::from(predictions))
        } else {
            Err("Model not trained".into())
        }
    }
}