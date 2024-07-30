use sentencepiece::SentencePieceProcessor;
use ct2rs::config::Config;
use ct2rs::translator::{Translator, TranslationOptions};

fn main() -> anyhow::Result<()> {
    //let t = Translator::new("../marianmt-en-zh", &Config::default())?;
    let t = Translator::new("/home/jason/.local/share/argos-translate/packages/translate-en_zh-1_9/model/", &Config::default())?;
    let encoder = SentencePieceProcessor::open("../marianmt-en-zh/source.spm")?;
    let decoder = SentencePieceProcessor::open("../marianmt-en-zh/target.spm")?;
    //let decoder = SentencePieceProcessor::open("/home/jason/.local/share/argos-translate/packages/translate-en_zh-1_9/sentencepiece.model")?;

    let argv: Vec<String> = std::env::args().collect();
    if argv.len() > 1 {
        let mut options: TranslationOptions<String> = Default::default();
        options.beam_size = 4;
        options.length_penalty = 0.2;
        options.max_batch_size = 32;
        options.num_hypotheses = 4;
        options.replace_unknowns = true;

        for input_source in argv.iter().skip(1) {
            println!("start encoding");
            let mut source: Vec<String> = encoder.encode(input_source)?.iter().map(|v| v.piece.to_string()).collect();
            source.push("</s>".to_string());
            println!("{:?}", source);
            println!("start translating");
            let res = t.translate_batch(&vec![source], &options)?;
            for r in res {
                if let Some(h) = r.hypotheses.get(0) {
                    println!("{:?}", decoder.decode_pieces(h)?);
                }
            }
        }
    }
    /*
    let mut source: Vec<String> = encoder.encode(
        "Thank you for sharing the data. I also tested the models you mentioned and discovered that the sentencepiece tokenizer does not append </s> to the end of the token list.",
    )?.iter().map(|v| v.piece.to_string()).collect();
     */
    Ok(())
}