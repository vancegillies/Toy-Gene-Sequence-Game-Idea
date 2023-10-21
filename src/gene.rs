use rand::Rng;

enum GeneChar {
    A,
    C,
    G,
    T,
    N,
    K,
    PLUS,
    MINUS,
}

impl GeneChar {
    pub fn value(&self) -> &'static str {
        match self {
            GeneChar::A => "A",
            GeneChar::C => "C",
            GeneChar::G => "G",
            GeneChar::T => "T",
            GeneChar::N => "N",
            GeneChar::K => "K",
            GeneChar::PLUS => "+",
            GeneChar::MINUS => "-",
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        self.value().to_string()
    }
}


struct GeneWeight {
    char: GeneChar,
    weight_one: f32, // first position
    weight_two: f32, // second position
}

impl GeneWeight {
    pub fn fixed() -> Vec<GeneWeight> {
        vec![
            GeneWeight { char: GeneChar::A, weight_one: 0.5, weight_two: 0.5 },
            GeneWeight { char: GeneChar::C, weight_one: 0.5, weight_two: 0.5 },
            GeneWeight { char: GeneChar::G, weight_one: 0.5, weight_two: 0.5 },
            GeneWeight { char: GeneChar::T, weight_one: 0.5, weight_two: 0.5 },
            GeneWeight { char: GeneChar::N, weight_one: 0.5, weight_two: 0.5 },
            GeneWeight { char: GeneChar::K, weight_one: 0.5, weight_two: 0.5 },
            GeneWeight { char: GeneChar::PLUS, weight_one: 0.0005, weight_two: 0.005 },
            GeneWeight { char: GeneChar::MINUS, weight_one: 0.0005, weight_two: 0.005 },
        ]
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.char.to_string(), self.weight_one, self.weight_two)
    }
}

fn weighted_random_select<'a>(strings: &'a Vec<&'a str>, weights: &Vec<f32>) -> Option<&'a str> {
    if strings.is_empty() || weights.is_empty() || strings.len() != weights.len() {
        return None; // Handle invalid input.
    }

    let total_weight = weights.iter().sum();
    let random_number = rand::thread_rng().gen_range(0.0..total_weight);
    let mut cumulative_weight = 0.0;

    for (i, weight) in weights.iter().enumerate() {
        cumulative_weight += *weight;
        if random_number <= cumulative_weight {
            return Some(strings[i]);
        }
    }

    None // Fallback case, should not be reached.
}

pub fn generate_sequence() -> String {
    let gene_weights = GeneWeight::fixed();
    let chars = gene_weights.iter().map(|gene_weight| gene_weight.char.value()).collect();
    let w_one = gene_weights.iter().map(|gene_weight| gene_weight.weight_one).collect();
    let w_two = gene_weights.iter().map(|gene_weight| gene_weight.weight_two).collect();

    let mut genes = String::new();

    for _ in 0..20 {
        let rand_one = weighted_random_select(&chars, &w_one);
        let rand_two = weighted_random_select(&chars, &w_two);

        if let (Some(one), Some(two)) = (rand_one, rand_two) {
            genes.push_str(one);
            genes.push_str(two);
        }
    }

    return genes;
}
