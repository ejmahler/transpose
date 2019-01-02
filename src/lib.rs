const BLOCK_SIZE: usize = 16;

// Transposes a block that might not necessarily be full-size. This will be used when we process the "remainder" of whatever is left over if the width or height isn't divisible by BLOCK_SIZE
unsafe fn transpose_block<T: Copy>(input: &[T], output: &mut [T], width: usize, height: usize, start_x: usize, start_y: usize, block_width: usize, block_height: usize) {
    for inner_x in 0..block_width {
        for inner_y in 0..block_height {
            let x = start_x + inner_x;
            let y = start_y + inner_y;

            let input_index = x + y * width;
            let output_index = y + x * height;

            *output.get_unchecked_mut(output_index) = *input.get_unchecked(input_index);
        }
    }
}

/// Given an array of size width * height, representing a flattened 2D array,
/// transpose the rows and columns of that 2D array into the output
pub fn transpose<T: Copy>(input: &[T], output: &mut [T], width: usize, height: usize) {
    assert_eq!(width*height, input.len());
    assert_eq!(width*height, output.len());

    let x_block_count = width / BLOCK_SIZE;
    let y_block_count = height / BLOCK_SIZE;

    let remainder_x = width - x_block_count * BLOCK_SIZE;
    let remainder_y = height - y_block_count * BLOCK_SIZE;

    for y_block in 0..y_block_count {
        for x_block in 0..x_block_count {
            unsafe {
                transpose_block(
                    input, output,
                    width, height,
                    x_block * BLOCK_SIZE, y_block * BLOCK_SIZE,
                    BLOCK_SIZE, BLOCK_SIZE,
                    );
            }
        }

        //if the width is not cleanly divisible by block_size, there are still a few columns that haven't been transposed
        if remainder_x > 0 {
            unsafe {
                transpose_block(
                    input, output, 
                    width, height, 
                    width - remainder_x, y_block * BLOCK_SIZE, 
                    remainder_x, BLOCK_SIZE);
            }
        }
    }

    //if the height is not cleanly divisible by BLOCK_SIZE, there are still a few columns that haven't been transposed
    if remainder_y > 0 {
        for x_block in 0..x_block_count {
            unsafe {
                transpose_block(
                    input, output,
                    width, height,
                    x_block * BLOCK_SIZE, height - remainder_y,
                    BLOCK_SIZE, remainder_y,
                    );
            }
        }

        //if the width is not cleanly divisible by block_size, there are still a few columns that haven't been transposed
        if remainder_x > 0 {
            unsafe {
                transpose_block(
                    input, output,
                    width, height, 
                    width - remainder_x, height - remainder_y, 
                    remainder_x, remainder_y);
            }
        }
    } 
}


#[cfg(test)]
mod unit_tests {
    use super::*;

    fn gen_data(width: usize, height: usize) -> Vec<usize> {
        (0 .. width * height).map(|x| x + 1).collect()
    }

    #[test]
    fn test_transpose() {
        let sizes = [
        	0, 1, 2,
        	BLOCK_SIZE - 1, BLOCK_SIZE, BLOCK_SIZE + 1, 
        	BLOCK_SIZE * 4 - 1, BLOCK_SIZE * 5, BLOCK_SIZE * 4 + 1
        	];

        for &width in &sizes {
            for &height in &sizes {
                let input = gen_data(width, height);
                let mut output = vec![0; width * height];

                transpose(&input, &mut output, width, height);

                for x in 0..width {
                    for y in 0..height {
                        assert_eq!(input[x + y * width], output[y + x * height], "x = {}, y = {}", x, y);
                    }
                }
            }
        }
    }
    }