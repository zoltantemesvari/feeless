#![forbid(unsafe_code)]
#![cfg_attr(feature = "deny_warnings", deny(warnings))]


use feeless::phrase::Language;
use feeless::Phrase;


fn main() {
    

    // Example phrase from https://docs.nano.org/integration-guides/key-management/#test-vectors
    let words = "edge defense waste choose enrich upon flee junk siren film clown finish
                 luggage leader kid quick brick print evidence swap drill paddle truly occur";

    // Generate the private key from the seed phrase.
    let phrase = Phrase::from_words(Language::English, words).unwrap();

    // First account with the password `some password`.
    let private_key = phrase.to_private(1, "some password").unwrap();
    let public_key = private_key.to_public().unwrap();
    println!("{}",public_key.to_string());
    let address = public_key.to_address();

    // The above three lines can be chained like this:
    println!("{}",address.to_string());

    // Sign a message.
    let message = "secret message!".as_bytes();
    let signature = private_key.sign(message).unwrap();

    println!("{}",signature.to_string());

    // Someone else can verify the message based on your address or public key.
    //address.to_public().verify(message, &signature).unwrap();
    let public_key = address.to_public();
    println!("{}",public_key.to_string());
    public_key.verify(message, &signature).unwrap();
}
