#[derive(Debug, Copy, Clone)]
enum Preference {
    Dominant,
    Recessive
}
#[derive(Debug, Copy, Clone)]
struct Allele {
    p: Preference,
    symbol: char
}
type Gene = (Allele, Allele);
type Genotype = Vec<Gene>;

fn possibility(a: Genotype) {}
fn generate_possible_children(a: &Genotype, b: &Genotype) {
    println!("{:?}", a);
    println!("{:?}", b);
    let genecount = a.len();
    for i in 0..genecount {
        for j in 0..genecount {
            println!("{:?}", vec![a[i], b[j]]);
        }
    }
}


/*
 * TT x Tt
 *
 * x | T | t
 * ---------
 * T|TT  | Tt
 */

fn main() {
    use Preference::*;
    // Rr capital = Red eye lowercase = black eye
    let aparent = vec![
        (
            Allele {
                p: Dominant,
                symbol: 'R'
            },
            Allele {
                p: Recessive,
                symbol: 'r'
            },
        ),
    ];
    let bparent = vec![
        (
            Allele {
                p: Recessive,
                symbol: 'r'
            },
            Allele {
                p: Recessive,
                symbol: 'r'
            },
        ),
    ];
    generate_possible_children(&aparent, &bparent);
}
