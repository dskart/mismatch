use ndarray::{ArrayBase, Axis, Dim, Ix2, ViewRepr};
use ort::{value::Tensor, Error};

use crate::app::session::Session;

impl Session {
    pub fn get_score(&self, word1: String, word2: String) -> anyhow::Result<i32> {
        if self.tokenizer.is_none() || self.ort_session.is_none() {
            anyhow::bail!("Tokenizer or ONNX session not initialized");
        }

        let (ids_tensor, offset_tensor) = self.get_embeddings_input(word1, word2)?;

        // Run the model.
        let sess_inputs = ort::inputs! {
            "input_ids" => ids_tensor,
            "offsets" => offset_tensor,
        }?;
        let outputs = self
            .ort_session
            .as_ref()
            .expect("ort_sessions is not initializedinitialized")
            .run(sess_inputs)?;

        // Extract our embeddings tensor and convert it to a strongly-typed 2-dimensional array.
        let embeddings = outputs[0].try_extract_tensor::<f32>()?.into_dimensionality::<Ix2>()?;
        let word1_embeddings = embeddings.index_axis(Axis(0), 0);
        let word2_embeddings = embeddings.index_axis(Axis(0), 1);

        let cosine_similarity = self.compute_cosine_similarity(word1_embeddings, word2_embeddings)?;

        // Convert from -1..1 to 0..1000, but invert the scale
        let score = ((1.0 - ((cosine_similarity + 1.0) / 2.0)) * 1000.0).round() as i32;

        anyhow::Ok(score)
    }

    fn get_embeddings_input(&self, word1: String, word2: String) -> anyhow::Result<(Tensor<i64>, Tensor<i64>)> {
        let inputs = vec![word1, word2];
        // https://github.com/MinishLab/model2vec/blob/main/README.md#integrations
        // Transformers.js example
        let encodings = self
            .tokenizer
            .as_ref()
            .expect("Tokenizer not initialized")
            .encode_batch(inputs.clone(), false)
            .map_err(|e| Error::new(e.to_string()))?;

        let input_ids: Vec<Vec<u32>> = encodings.iter().map(|enc| enc.get_ids().to_vec()).collect();
        let mut offsets = vec![0];
        let mut cumsum = 0;
        for ids in &input_ids[..input_ids.len() - 1] {
            cumsum += ids.len();
            offsets.push(cumsum as i64);
        }

        let flattened_input_ids: Vec<i64> = input_ids.into_iter().flatten().map(|x| x as i64).collect();
        let ids_tensor = Tensor::from_array(([flattened_input_ids.len()], flattened_input_ids.into_boxed_slice()))?;
        let offset_tensor = Tensor::from_array(([offsets.len()], offsets.into_boxed_slice()))?;

        Ok((ids_tensor, offset_tensor))
    }

    fn compute_cosine_similarity(
        &self,
        word1_embeddings: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>,
        word2_embeddings: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>,
    ) -> anyhow::Result<f32> {
        // Calculate cosine similarity
        let dot_product: f32 = word1_embeddings
            .iter()
            .zip(word2_embeddings.iter())
            .map(|(a, b)| a * b)
            .sum();
        let norm1: f32 = word1_embeddings.iter().map(|a| a * a).sum::<f32>().sqrt();
        let norm2: f32 = word2_embeddings.iter().map(|a| a * a).sum::<f32>().sqrt();
        let cosine_similarity = dot_product / (norm1 * norm2);

        anyhow::Ok(cosine_similarity)
    }
}

#[cfg(test)]
mod tests {
    use crate::app::{model::*, tests::TestApp, *};
    use anyhow::Result;

    #[test]
    fn test_get_score_similarity() -> Result<()> {
        let session = TestApp::new(Some(Config {
            model: ModelType::PotionBase8M,
        }))
        .new_session();

        // Test identical words
        let score1 = session.get_score("cat".to_string(), "cat".to_string())?;
        assert_eq!(score1, 0); // Identical words should have score 0

        // Test similar words
        let score2 = session.get_score("cat".to_string(), "kitten".to_string())?;
        assert!(score2 > 0 && score2 < 500); // Similar words should have mid-range score
        Ok(())
    }

    #[test]
    fn test_get_embeddings_input() -> Result<()> {
        let session = TestApp::new(Some(Config {
            model: ModelType::PotionBase8M,
        }))
        .new_session();

        let (ids_tensor, offset_tensor) = session.get_embeddings_input("cat".to_string(), "kitten".to_string())?;
        assert_eq!(ids_tensor.shape()?.as_slice(), &[2]);
        assert_eq!(offset_tensor.shape()?.as_slice(), &[2]);
        Ok(())
    }

    #[test]
    fn test_compute_cosine_similarity() -> Result<()> {
        let session = TestApp::new(Some(Config {
            model: ModelType::PotionBase8M,
        }))
        .new_session();

        let word1_embeddings = ndarray::arr1(&[1.0, 2.0, 3.0]);
        let word2_embeddings = ndarray::arr1(&[2.0, 3.0, 4.0]);
        let cosine_similarity = session.compute_cosine_similarity(word1_embeddings.view(), word2_embeddings.view())?;
        assert_eq!(cosine_similarity, 0.9925833);
        Ok(())
    }
}
