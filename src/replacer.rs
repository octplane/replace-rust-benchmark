extern crate chrono;

use std::str;
use chrono::*;
use std::io::prelude::*;
use std::fs::File;

fn bench() {
    // I have left, keys and values and the file read in this function
    // because in real life, this will come from the outside Racket Caller
    // and I wanted to simulate the C->Rust slight overhead by leaving these
    // declaration and IO here.

    let keys= vec!["MÉCAN. DES FLUIDES.", "qqc.?", " :", " ds ", "rad.", "ANAT.", "dr.", "relig.", "PHYS. NUCL.", "CHRIST.", ", anthropol.", "dict. gén.", "REM. GÉN.", "élém.", "Rem.", ",,", "Hist.", "<sup>e</sup>&nbsp;s.", ",</span>", "NAV.", "phon.", "PHILOS.", "JUST.", "AUTOMOB.", "Corresp.", "v. aussi", "Fréq.", "P. métaph.", "Correspondantà", "p. oppos.", "technol.", "corresp.", "PATHOL.", "CONSTR.", "Empr.", "Forme graph.", "adv.", "attest.", "inf.", "THÉOL. CHRÉT.", "ÉTYMOL.", "Inf.", "inus.", "<div class=\"tlf_paraputir\">-", "compl.", "loc.", "GÉOGR.", "littér.", "Loc. proverbiales", "Loc.", "P. oppos.", "PHYSIOL.", "néol.", "Ac. Compl.", "TECHN. D'INFORM.", "P. méton.", "OCÉANOGR.", "concr.", "fém.", "sing.", "ADMIN.", "REM.", "cf.", "Gén. ", "pers.", "<span class=\"tlf_cdate\">, ", "qqn", "trans.", "apr.", " p.", " v.", "CHIM.", ", zool.", "BOT.", "``", "angl.", "TEXT.", "aéron.", "att.", "lat. vulg.", "constr.", "partic.", "dér.", "arg.", "électr.", "Arg.", "plur.", "abs.", "IMPR.", "Chimie ANC.", "abstr.", "HÉRALD.", "TECHNOL.", "ex.", "Étymol.", "MAR.", "lang.", "P. ext.", "P. hyperb.", "STAT.", "Loc. fig.", "dict.", "Empl.", "BIOL.", "fam.", "préf.", "synon.", "Dér", "qqf.", "P. anal.", "gén.", "Expr.", "Synon.", "Orth.", "masc.", "<div class=\"tlf_parothers\"></div>", "Fréquence abs. littér.", "CHORÉGR.", "rel.", "CHIR.", "Besch.", "MÉTÉOR.", "DÉR.", "<div id=\"art", "BBg.", "SYNT.", "métaph.", "V. ", "LING.", "péj.", "</sup>", "suff.", "Région.", "d'obj.", "GÉOMORPHOL.", "transcr.", "Bbg", "PRONONC.", "Anglo-amér.", "vx", "adj.", "GÉOL.", "Subst.", "Vx", "Pronom.", "au fig.", "Ac.", "subst.", "suj.", "DR.", "Prononc.", "docum.", "HIST.", "a.fr.", "pop.", "réfl.", "ÉLECTR.", "pronom.", "Histoire NAT.", "constr. adv.", "Au fig.", "BBG."];

    let values= vec!["Mécanique des fluides", "quelque chose", "&nbsp;:", " dans ", "radical", "Anatomie", "droit", "religions", "Physique nucléaire", "Christianisme", "anthropologie", "dictionnaires généraux", "Remarques générales", "élément", "Remarque", "«", "Histoire", "<sup>ème</sup>&nbsp;siècle", "</span>,&nbsp;", "Navigation", "phonétique", "Philosophie", "Justice", "Automobile", "Correspondant", "voir aussi", "Fréquence", "Par métaphore", "Correspondant à", "par opposition", "technologique", "correspondant", "Pathologie", "Construction", "Emprunté", "Forme graphique", "adverbe", "apparition attestée", "infinitif", "Théologie Chrétienne", "Étymologie", "Infinitif", "inusité", "<div class=\"tlf_paraputir\">", "complément", "locution", "Géographie", "littéraire", "Locutions proverbiales", "Locution", "Par opposition", "Physiologie", "néologisme", "Avec Complément", "Technologies de l'information", "Par métonymie", "Océanographie", "concret", "féminin", "singulier", "Administration", "Remarque", "confer", "Généralement ", "personne", ", <span class=\"tlf_cdate\">", "quelqu'un", "transitif", "après", " page ", " voir", "Chimie", "zoologie", "Botanique", "»", "anglophone", "Textile", "aéronautique", "attesté", "Latin Vulgaire", "construction", "particulier", "dérivé", "argotique", "électricité", "Argot", "pluriel", "absolue", "Imprimerie", "Chimie Ancienne", "abstrait", "Héraldique", "Technologie", "exemple", "Étymologie", "Marine", "langues", "Par extension", "Par hyperbole", "Statistiques", "Locution figurative", "dictionnaires", "Emploi", "Biologie", "familier", "préfixe", "synonyme", "Dérivé", "quelquefois", "Par analogie", "généralement", "Expression", "Synonyme", "Orthographe", "masculin", "", "Fréquence absolue littéraire", "Chorégraphie", "relative", "Chirurgie", "Bescherelles", "Météorologie", "DÉRIVÉS", "<div class=\"article\" id=\"art", "Bibliographie", "SYNTAGMES", "métaphorique", "Voir ", "Linguistique", "péjoratif", "</sup>&nbsp;", "suffixe", "Régional", "d'objet", "Géomorphologie", "transcription", "Bibliographie", "Prononciation", "anglo-américain", "vieux", "adjectif", "Géologie", "Substantif", "Vieux", "Pronominal", "au figuré", "Académie française", "substantif", "sujet", "Droit", "Prononciation", "documentation", "Histoire", "ancien français", "populaire", "réfléchi", "Électricité", "pronominal", "Histoire Naturelle", "construction adverbiale", "Au figuré, ", "Bibliographie"];


    let mut f = File::open("test_data.txt").unwrap();
    let mut target = String::new();
    f.read_to_string(&mut target).unwrap();

    replace_(keys, values, target);
}


#[inline(never)]
fn replace_(keys: Vec<&str>, values: Vec<&str>, target: String) -> String {
    //let before: DateTime<UTC> = UTC::now();
    let mut rvalue = target;
    for (ix, k) in keys.iter().enumerate() {
        let v = values[ix];
        rvalue = rvalue.replace(k, v);
    }
    //let after: DateTime<UTC> = UTC::now();
    //println!("[RUST] Substitution took {}ms\n", (after - before).num_milliseconds());
    rvalue
}

// cargo run --release
fn main() {
    for _ in 0..1000 {
        bench();
    }
}
