use std::io;
fn main() {
    println!("Input a string:");
    let mut english: String = String::new();
    // let _original = english.clone();
    io::stdin()
        .read_line(&mut english)
        .expect("Unable to read Stdin");
    println!("Enable Thornmode? [Y/n] ");
    let mut thornmode: String = String::new();
    io::stdin()
        .read_line(&mut thornmode)
        .expect("Unable to read Stdin");
    println!(" ");
    while ! thornmode.contains("Y") && ! thornmode.contains("y") && ! thornmode.contains("n") && ! thornmode.contains("N") {
        println!("That didn't work. Enable thornmode? [Y/n] ");
        let mut thornmode: String = String::new();
        io::stdin()
            .read_line(&mut thornmode)
            .expect("Unable to read Stdin");
        println!("{}",thornmode);
        if thornmode.contains("Y") || thornmode.contains("y") || thornmode.contains("n") || thornmode.contains("N") {
            break;
        }
    }
    if thornmode.contains("Y") || thornmode.contains("y") {
        println!("Thornmode is enabled. Letter 'Thorn' will be used.");
        english = english.replace("th","þ");
    } else if thornmode.contains("n") || thornmode.contains("N") {
        println!("Thornmode is disabled. Letter 'Eth' will be used.");
        english = english.replace("th","ð");
    } else {
        println!("Thornmode toggle could not be determined. The conversion will continue, but \"th\" substrings are not going to be affected.\n");
    }
    let replacements = vec![
        ("thom","tom"),
        ("coope","cöpe"),
        ("co-op","cöp"),
        ("alk","ak"),
        ("sh","ʃ"),
        ("tio","ʃo"),
        ("sio","ʃo"),
        ("sure","ʃur"),
        ("ll","l"),
        // ("le","el"),
        ("co","ko"),
        ("cu","ku"),
        ("ca","ka"),
        ("ck","k"),
        ("ic","ik"),
        ("cr","kr"),
        ("ci","si"),
        ("ce","se"),
        ("cy","sy"),
        ("ch","c"),
        ("ikh","ic"),
        ("kn","gn"),
        ("ec","ek"),
        ("act","akt"),
        ("cem","kem"),
        ("whik","whic"),
        ("nge","nje"),
        ("ng","ŋ"),
        ("nk","ŋk"),
        ("ph","f"),
        ("ause","auz"),
        ("ause","auz"),
        ("ouse","aus"),
        ("cough","koff"),
        ("laugh","laff"),
        ("enough","enuf"),
        ("tough","tuff"),
        ("ough","o"),
        ("gh",""),
        ("exa","egza"),
        ("exi","egzi"),
        ("ax","aks"),
        ("ox","oks"),
        ("ux","uks"),
        ("ix","iks"),
        ("ex","eks"),
        ("x","z"),
        ("oo","u"),
        ("þro","þru"),
        ("þruw","þrow"),
        ("of","ov"),
        ("uld","ud"),
        ("kss","ks"),
        ("idk","idg"),
        ("ture","cur"),
        ("æcʃ","ækʃ"),
        ("wið","wiþ"),
        ("arsitekkur","arkitekcur"),
        ("geo","jeo"),
        ("rge","rje"),
    ];
    for (a,b) in replacements {
        english = english.replace(a,b);
    }
    println!("This is not a perfect system, and manual adjustments may be needed.\n\nYour output is:");
    println!("{}",english);
}

