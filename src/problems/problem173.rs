use anyhow::Result;

pub fn p() -> Result<String> {
    /*
    Using up to one million tiles how many different "hollow" square laminae can be formed?
    Problem 173

    We shall define a square lamina to be a square outline with a square "hole" so that the shape possesses vertical and
    horizontal symmetry. For example, using exactly thirty-two square tiles we can form two different square laminae

    With one-hundred tiles, and not necessarily using all of the tiles at one time, it is possible to form forty-one
    different square laminae.

    Using up to one million tiles how many different square laminae can be formed?

                2*(i * ((3+(j-1) + 2*i)) + j * (2 * i)
    1*1 --> 1x : 1*3+1*3+1*2 = 8   2x : 2*5+2*5+1*4   = 24  3x : 3*7+3*7+1*6      = 48    4x : 4*9+4*9+1*8        = 80
    2*2 --> 1x : 1*4+1*4+2*2 = 12  2x : 2*6+2*6+2*4   = 32  3x : 3*8+3*8+2*6      = 60    4x : 4*10+4*10+2*8      = 96
    3*3 --> 1x : 1*5+1*5+3*2 = 16  2x : 2*7+2*7+3*4   = 40  3x : 3*9+3*9+3*6      = 72    4x : 4*11+4*11+3*8      = 112
    4*4 --> 1x : 1*6+1*6+4*2 = 20  2x : 2*8+2*8+4*4   = 48  3x : 3*10+3*10+4*6    = 84    4x : 4*12+4*12+4*8      = 128
    5*5 --> 1x : 1*7+1*7+5*2 = 24  2x : 2*9+2*9+5*4   = 56  3x : 3*11+3*11+5*6    = 96    4x : 4*13+4*13+5*8      = 144
    6*6 --> 1x : 1*8+1*8+6*2 = 28  2x : 2*10+2*10+6*4 = 64  3x : 3*12+3*12+6*6    = 108   4x : 4*14+4*14+6*8      = 160
    7*7 --> 1x : 1*9+1*9+7*2 = 32  2x : 2*11+2*11+7*4 = 72  3x : 3*13+3*13+7*6    = 120   4x : 4*15+4*15+7*8      = 176
    */
    const MAX_INDEX: u64 = 1e6 as u64;
    let mut nb_ways: u64 = 0;
    for i in 1..MAX_INDEX {
        nb_ways += (1..MAX_INDEX)
            .map_while(|j| {
                let nb_tiles = 2 * (i * (3 + (j - 1) + 2 * (i - 1))) + j * (2 * i);
                (nb_tiles <= MAX_INDEX).then_some(0)
            })
            .count() as u64;
    }
    Ok(nb_ways.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(p().unwrap(), "1572729");
    }
}
