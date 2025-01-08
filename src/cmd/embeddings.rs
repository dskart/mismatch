use crate::cmd::Config;
use ndarray::{Axis, Ix2};
use ort::{
    session::{builder::GraphOptimizationLevel, Session},
    value::Tensor,
    Error,
};
use std::io::{self, Write};
use std::path::Path;
use tokenizers::Tokenizer;

pub const CMD_NAME: &str = "embeddings";

pub fn cmd() -> clap::Command {
    clap::Command::new(CMD_NAME).about("run embeddings")
}

pub async fn run(_config: Config, _args: &clap::ArgMatches) -> anyhow::Result<()> {
    let mut inputs = Vec::new();
    // let inputs = vec!["I enjoy taking long walks along the beach with my dog.".to_string()];
    for i in 1..=2 {
        print!("Enter word {}: ", i);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        inputs.push(input.trim().to_string());
    }

    ort::init().with_name("sbert").commit()?;

    // Load our model
    let session = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level1)?
        .with_intra_threads(1)?
        .commit_from_file(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("models/potion-base-8M")
                .join("potion-base-8M.onnx"),
        )?;

    let tokenizer = Tokenizer::from_file(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("models/potion-base-8M")
            .join("tokenizer.json"),
    )
    .expect("Failed to load tokenizer");

    // https://github.com/MinishLab/model2vec/blob/main/README.md#integrations
    // Transformers.js example

    // Encode our input strings. `encode_batch` will pad each input to be the same length.
    let encodings = tokenizer
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
    let ids_tensor = Tensor::from_array((
        [flattened_input_ids.len()],
        flattened_input_ids.into_boxed_slice(),
    ))?;
    let offset_tensor = Tensor::from_array(([offsets.len()], offsets.into_boxed_slice()))?;

    // Run the model.
    let sess_inputs = ort::inputs! {
        "input_ids" => ids_tensor,
        "offsets" => offset_tensor,
    }?;
    let outputs = session.run(sess_inputs)?;

    // Extract our embeddings tensor and convert it to a strongly-typed 2-dimensional array.
    let embeddings = outputs[0]
        .try_extract_tensor::<f32>()?
        .into_dimensionality::<Ix2>()
        .unwrap();

    println!("Similarity for '{}'", inputs[0]);
    let query = embeddings.index_axis(Axis(0), 0);
    for (embeddings, sentence) in embeddings.axis_iter(Axis(0)).zip(inputs.iter()).skip(1) {
        // Calculate cosine similarity against the 'query' sentence.
        let dot_product: f32 = query
            .iter()
            .zip(embeddings.iter())
            .map(|(a, b)| a * b)
            .sum();
        println!("\t'{}': {:.1}%", sentence, dot_product * 100.);
    }

    anyhow::Ok(())
}
