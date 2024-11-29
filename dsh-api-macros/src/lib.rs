use proc_macro::TokenStream as TokenStream1;
use proc_macro2::token_stream::IntoIter;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn get_by_tenant(input: TokenStream1) -> TokenStream1 {
  get_by_tenant_impl(parse_macro_input!(input)).into()
}

fn get_by_tenant_impl(input: TokenStream) -> TokenStream {
  println!("{:#?}", input);
  let mut input_iter: IntoIter = input.into_iter();

  let (internal_method_name, method_name): (TokenStream, TokenStream) = match input_iter.next().unwrap() {
    TokenTree::Ident(ident) => {
      let method_name = ident.to_string();
      (method_name.parse().unwrap(), method_name.replace("_by_tenant", "").parse().unwrap())
    }
    _ => {
      eprintln!(">>>>>>>>>>>>> 1");
      panic!("expected Ident")
    }
  };

  // Consume comma separator
  input_iter.next();

  let mut arguments = vec![];
  loop {
    match input_iter.next().unwrap() {
      TokenTree::Group(_) => {
        eprintln!(">>>>>>>>>>>>> 2");
        panic!()
      }
      TokenTree::Ident(ident) => arguments.push(ident.to_string()),
      TokenTree::Punct(_) => break,
      TokenTree::Literal(_) => {
        eprintln!(">>>>>>>>>>>>> 3");
        panic!()
      }
    }
  }

  println!(">>>>>>>>>> {:?}", arguments);

  let method_arguments = arguments.iter().map(|a| format!("{}: &str", a).parse().unwrap()).collect::<Vec<TokenStream>>();
  let call_arguments = arguments.iter().map(|a| a.to_string().parse().unwrap()).collect::<Vec<TokenStream>>();

  let result_ok_type: TokenStream = match input_iter.next().unwrap() {
    TokenTree::Ident(ident) => ident.to_string().parse().unwrap(),
    other => {
      eprintln!(">>>>>>>>>>>>> 4 {:?}", other);
      panic!()
    }
  };

  // Consume comma separator
  input_iter.next();

  let _comments = input_iter.collect::<TokenStream>();
  println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

  quote!(
    pub async fn #method_name(&self, #(#method_arguments),*) -> DshApiResult<#result_ok_type> {
      self
        .process(self.#internal_method_name(self.tenant_name()#(#call_arguments),*, self.token()).await)
        .map(|result| result.1)
    }
  )
}
