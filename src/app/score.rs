use ndarray::{Axis, Ix2};
use ort::{value::Tensor, Error};

use crate::app::App;

impl App {
    pub fn get_score(&self, word1: String, word2: String) -> anyhow::Result<i32> {
        if self.tokenizer.is_none() || self.ort_session.is_none() {
            anyhow::bail!("Tokenizer or ONNX session not initialized");
        }

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

        // Since there is only one dimension, just compute dot product
        let word1_embeddings = embeddings.index_axis(Axis(0), 0);
        let word2_embeddings = embeddings.index_axis(Axis(0), 1);

        // Calculate cosine similarity and convert to 0-1000 range
        let dot_product: f32 = word1_embeddings
            .iter()
            .zip(word2_embeddings.iter())
            .map(|(a, b)| a * b)
            .sum();
        let norm1: f32 = word1_embeddings.iter().map(|a| a * a).sum::<f32>().sqrt();
        let norm2: f32 = word2_embeddings.iter().map(|a| a * a).sum::<f32>().sqrt();
        let cosine_similarity = dot_product / (norm1 * norm2);
        // Convert from -1..1 to 0..1000, but invert the scale
        let score = ((1.0 - ((cosine_similarity + 1.0) / 2.0)) * 1000.0).round() as i32;

        anyhow::Ok(score)
    }
}
