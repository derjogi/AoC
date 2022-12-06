use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta, Lit, Ident};

// Specifies a macro that is used in each day to run the main() method with the
// input for the day. Inspired and modified from https://github.com/AxlLind/AdventOfCode2021

#[proc_macro_attribute]
pub fn main(args: TokenStream, real_function: TokenStream) -> TokenStream {
    // Lets us call main() functions with a `day` as parameter
    // * args is a specific argument passed to the macro,
    // * 'real_function' is (I believe?) the actual main method
    //   (or whatever comes after the macro, passed in here as parameter).

    // Construct the correct path from the macro argument:
    let input_path = match &parse_macro_input!(args as AttributeArgs)[..] {
        [NestedMeta::Lit(Lit::Int(day))] => format!("../../inputs/day{}/main.in", day.token().to_string()),
        _ => panic!("Expected one integer argument for the day of this exercise"),
    };

    // I think this creates aoc_solution as kind of a fake function that we just refer to, but that
    // gets expanded to whatever the _main()_ function after the macro invocation is?
    let mut aoc_solution = parse_macro_input!(real_function as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    // Not quite sure how TokenStream is used, but seems like this `main()` function
    // is what actually gets executed, and the aoc_solution (=real_function) is called from within.
    let tokens = quote! {
        // #var = interpolation of runtime variables into the quoted! tokens
        const INPUT: &str = include_str!(#input_path);
        #aoc_solution
        fn main() {
          aoc_solution(INPUT.trim_end());
        }
    };
    TokenStream::from(tokens)
}
