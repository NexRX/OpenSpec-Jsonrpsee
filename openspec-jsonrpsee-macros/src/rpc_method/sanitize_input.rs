use syn::ItemFn;

pub fn generate(input: &syn::ItemFn) -> syn::ItemFn {
    let mut input = input.clone();
    fn_arg_drop_attr_context(&mut input);
    input
}

/// remove #[context] on function args (if any)
fn fn_arg_drop_attr_context(input: &mut ItemFn) {
    let mut new_inputs = syn::punctuated::Punctuated::new();
    for arg in input.sig.inputs.iter() {
        let arg = match arg {
            syn::FnArg::Typed(pat_type) => {
                let mut pat_type = pat_type.clone();
                pat_type
                    .attrs
                    .retain(|attr| !attr.path().is_ident("context"));
                syn::FnArg::Typed(pat_type)
            }
            syn::FnArg::Receiver(receiver) => {
                // syn::Receiver does have attrs, but we keep as-is
                syn::FnArg::Receiver(receiver.clone())
            }
        };
        new_inputs.push(arg);
    }
    input.sig.inputs = new_inputs;
}
