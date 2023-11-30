use proc_macro::TokenStream;
use quote::quote;

// Again, huge thanks to andi_makes
// this is basically a copy of his macros, with very slight modifications.

#[proc_macro]
pub fn mod_days(_input: TokenStream) -> TokenStream {
    let mut res = String::new();
    for i in 1..26 {
        res += &format!("mod d{:02};", i);
    }
    res.parse().unwrap()
}

/*#[proc_macro]
pub fn match_and_run_day(_input: TokenStream) -> TokenStream {
    let mut res = "match day {".to_string();
    for i in 1..26 {
        res += &format!("{} => Day::<{}>::run_timed(input),", i, i);
    }
    res += "_ => panic!(\"Days out of Bounds! No presents for you!\")};";
    res.parse().unwrap()
}*/

#[proc_macro]
pub fn match_and_run_day_both(_input: TokenStream) -> TokenStream {
    let r = 1_u8..26; // == [1,25]
    let res = quote! {
        match day {
            #(#r => {
                Day::<#r>::run_timed(input.trim_end())
            })*
            _ => panic!("Days out of Bounds! No presents for you!"),
        }
    };
    res.into()
}

#[proc_macro]
pub fn match_and_run_day_one(_input: TokenStream) -> TokenStream {
    let r = 1_u8..26; // == [1,25]
    let res = quote! {
        match day {
            #(#r => {
                Day::<#r>::run_one_timed(input.trim_end())
            })*
            _ => panic!("Days out of Bounds! No presents for you!"),
        }
    };
    res.into()
}

#[proc_macro]
pub fn match_and_run_day_two(_input: TokenStream) -> TokenStream {
    let r = 1_u8..26; // == [1,25]
    let res = quote! {
        match day {
            #(#r => {
                Day::<#r>::run_two_timed(input.trim_end())
            })*
            _ => panic!("Days out of Bounds! No presents for you!"),
        }
    };
    res.into()
}

#[proc_macro]
pub fn match_and_test_day_both(_input: TokenStream) -> TokenStream {
    let r = 1_u8..26; // == [1,25]
    let res = quote! {
        match day {
            #(#r => {
                Day::<#r>::test()
            })*
            _ => panic!("Days out of Bounds! No presents for you!"),
        }
    };
    res.into()
}

#[proc_macro]
pub fn match_and_test_day_one(_input: TokenStream) -> TokenStream {
    let r = 1_u8..26; // == [1,25]
    let res = quote! {
        match day {
            #(#r => {
                Day::<#r>::test_one()
            })*
            _ => panic!("Days out of Bounds! No presents for you!"),
        }
    };
    res.into()
}

#[proc_macro]
pub fn match_and_test_day_two(_input: TokenStream) -> TokenStream {
    let r = 1_u8..26; // == [1,25]
    let res = quote! {
        match day {
            #(#r => {
                Day::<#r>::test_two()
            })*
            _ => panic!("Days out of Bounds! No presents for you!"),
        }
    };
    res.into()
}
