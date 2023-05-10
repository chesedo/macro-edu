use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Ident, Path, Token,
};

#[proc_macro]
pub fn copy(item: TokenStream) -> TokenStream {
    let model = parse_macro_input!(item as Model);

    let output = quote!(#model);

    output.into()
}

// input -(syn)-> model -(quote)-> output

#[derive(Debug, PartialEq)]
struct Model {
    base_type: Path,
    first_arrow: Token!(->),
    target_type: Path,
    colon: Token!(:),
    base_ident: Ident,
    second_arrow: Token!(->),
    target_ident: Ident,
    brace_token: Brace,
    fields: Punctuated<Field, Token!(,)>,
}

impl Parse for Model {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            base_ident: input.parse()?,
            first_arrow: input.parse()?,
            target_ident: input.parse()?,
            colon: input.parse()?,
            base_type: input.parse()?,
            second_arrow: input.parse()?,
            target_type: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse, Token!(,))?,
        })
    }
}

impl ToTokens for Model {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // Business logic

        let &Self {
            base_type: _, // TODO: confirm the type is correct / exists. Maybe `(sarah as Type)`?
            first_arrow: _,
            target_type,
            colon: _,
            base_ident,
            second_arrow: _,
            target_ident,
            brace_token: _,
            fields,
        } = &self;

        let (fields_base, fields_target): (Vec<_>, Vec<_>) = fields
            .iter()
            .map(|f| (f.base.clone(), f.target.clone()))
            .unzip();

        let output = quote! {
            let #target_ident = #target_type {
                #(#fields_target: #base_ident.#fields_base,)*
            };
        };

        tokens.extend(output);
    }
}

#[derive(Debug, PartialEq)]
struct Field {
    base: Ident,
    arrow: Token!(->),
    target: Ident,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            base: input.parse()?,
            arrow: input.parse()?,
            target: input.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Field, Model};
    use pretty_assertions::assert_eq;
    use quote::quote;
    use syn::{parse_quote, punctuated::Punctuated};

    #[test]
    fn field_input() {
        let actual: Field = parse_quote!(name -> first_name);
        let expected = Field {
            base: parse_quote!(name),
            arrow: Default::default(),
            target: parse_quote!(first_name),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn model_input() {
        let actual: Model = parse_quote!(
            sarah -> hooman: BigPerson -> Hooman {
                name -> first_name,
                age -> years_old,
            }
        );
        let mut expected = Model {
            base_type: parse_quote!(BigPerson),
            first_arrow: Default::default(),
            target_type: parse_quote!(Hooman),
            colon: Default::default(),
            base_ident: parse_quote!(sarah),
            second_arrow: Default::default(),
            target_ident: parse_quote!(hooman),
            brace_token: Default::default(),
            fields: Punctuated::new(),
        };

        expected.fields.push(parse_quote!(name -> first_name));
        expected.fields.push(parse_quote!(age -> years_old));
        expected.fields.push_punct(Default::default());

        assert_eq!(actual, expected);
    }

    #[test]
    fn model_output() {
        let mut input = Model {
            base_type: parse_quote!(BigPerson),
            first_arrow: Default::default(),
            target_type: parse_quote!(Hooman),
            colon: Default::default(),
            base_ident: parse_quote!(sarah),
            second_arrow: Default::default(),
            target_ident: parse_quote!(hooman),
            brace_token: Default::default(),
            fields: Punctuated::new(),
        };
        input.fields.push(parse_quote!(name -> first_name));
        input.fields.push(parse_quote!(age -> years_old));

        let actual = quote!(#input);
        let expected = quote!(
            let hooman = Hooman {
                first_name: sarah.name,
                years_old: sarah.age,
            };
        );

        assert_eq!(actual.to_string(), expected.to_string());
    }
}
