pub mod tsp;

pub mod strings {
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::str;

    pub fn search_min_window<'b>(pat: &str, string: &'b str) -> &'b str {
        let mut pat_dic: HashMap<u8, u32> = HashMap::new();
        let mut string_dic: HashMap<u8, u32> = HashMap::new();

        for b in pat.as_bytes() {
            let count = pat_dic.entry(*b).or_insert(0);
            *count += 1;
        }

        let mut start = 0;
        let mut start_index: usize = 0;
        let mut min_len: usize = 1 << 30;
        let mut count = 0;
        let mut index = 0;

        for b in string.as_bytes() {
            let freq = string_dic.entry(*b).or_insert(0);
            *freq += 1;

            if let Some(v) = pat_dic.get(b) {
                if v >= freq {
                    count += 1;
                }
            }

            if count == pat.len() {
                loop {
                    let ch = string.as_bytes()[start];
                    let freq_pat = pat_dic.entry(ch).or_default();
                    let freq_str = string_dic.entry(ch).or_default();
                    if *freq_pat == 0 || freq_str > freq_pat {
                        *freq_str -= 1;
                        start += 1;
                    } else {
                        break;
                    }
                }

                let window_size = index - start + 1;
                if min_len > window_size {
                    min_len = window_size;
                    start_index = start as usize;
                }
            }

            index += 1;
        }
        let end = start_index + min_len;
        str::from_utf8(&string.as_bytes()[start_index..end]).unwrap()
    }

    pub fn the_8_queen_puzzle_nw_solution() -> [usize; 8] {
        let x = RefCell::new([0usize; 10]);
        let col: Cell<usize> = Cell::new(0);
        let row: Cell<usize> = Cell::new(0);
        let safe = Cell::new(false);

        let consider_first_column = || {
            col.set(1);
            row.set(0);
        };

        let consider_next_column = || {
            x.borrow_mut()[col.get()] = row.get();
            col.set(col.get() + 1);
            row.set(0);
        };

        let advance_pointer = || {
            row.set(row.get() + 1);
        };

        let last_square = || -> bool { row.get() == 8 };

        let last_col_done = || -> bool { col.get() > 8 };

        // auxiliary data structure to speed up testSquare
        let row_is_free = RefCell::new([true; 9]); // range [1, 8]
        let diagonal_1_is_free = RefCell::new([true; 17]); // range [2, 16]
        let diagonal_2_is_free = RefCell::new([true; 15]); // range [-7, 7]

        let test_square = || {
            safe.set(
                row_is_free.borrow()[row.get()]
                    && diagonal_1_is_free.borrow()[row.get() + col.get()]
                    && diagonal_2_is_free.borrow()[7 + row.get() - col.get()],
            );
        };

        let try_column = || loop {
            advance_pointer();
            test_square();
            if safe.get() || last_square() {
                break;
            }
        };

        let set_queen = || {
            row_is_free.borrow_mut()[row.get()] = false;
            diagonal_1_is_free.borrow_mut()[row.get() + col.get()] = false;
            diagonal_2_is_free.borrow_mut()[7 + row.get() - col.get()] = false;
        };

        let remove_queen = || {
            row_is_free.borrow_mut()[row.get()] = true;
            diagonal_1_is_free.borrow_mut()[row.get() + col.get()] = true;
            diagonal_2_is_free.borrow_mut()[7 + row.get() - col.get()] = true;
        };

        let reconsider_prior_col = || {
            col.set(col.get() - 1);
            row.set(x.borrow_mut()[col.get()]);
        };

        let regress_out_of_first_col = || -> bool { col.get() < 1 };

        let regress = || {
            reconsider_prior_col();
            if !regress_out_of_first_col() {
                remove_queen();
                if last_square() {
                    reconsider_prior_col();
                    if !regress_out_of_first_col() {
                        remove_queen();
                    }
                }
            }
        };
        // algorithm
        consider_first_column();
        loop {
            try_column();
            if safe.get() {
                set_queen();
                consider_next_column();
            } else {
                regress();
            }

            if last_col_done() || regress_out_of_first_col() {
                break;
            }
        }

        let mut result = [0usize; 8];
        result.copy_from_slice(&x.borrow()[1..9]);

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::strings::*;

    #[test]
    fn test_search_min_window() {
        struct Data {
            string: &'static str,
            pat: &'static str,
            expected: &'static str,
        }

        let cases = vec![
            Data {
                string: "ABDOECBOKABABKC",
                pat: "ABC",
                expected: "ABKC",
            },
            Data {
                string: "this is a test string",
                pat: "tist",
                expected: "t stri",
            },
            Data {
                string: "geeksforgeeks",
                pat: "ork",
                expected: "ksfor",
            },
        ];

        for data in cases {
            assert_eq!(search_min_window(data.pat, data.string), data.expected);
        }
    }

    #[test]
    fn test_8_queen_puzzle_nw_solution() {
        assert_eq!(the_8_queen_puzzle_nw_solution(), [1, 5, 8, 6, 3, 7, 2, 4])
    }

}
