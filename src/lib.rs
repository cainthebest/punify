use idna::{domain_to_ascii, domain_to_ascii_strict, domain_to_unicode, punycode};
use neon::prelude::*;

pub fn to_ascii(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?.value(&mut cx);
    match domain_to_ascii(&input) {
        Ok(encoded) => Ok(cx.string(encoded)),
        Err(e) => cx.throw_error(format!("Error encoding string: {:?}", e)),
    }
}

pub fn to_ascii_strict(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?.value(&mut cx);
    match domain_to_ascii_strict(&input) {
        Ok(encoded) => Ok(cx.string(encoded)),
        Err(e) => cx.throw_error(format!("Error in strict encoding: {:?}", e)),
    }
}

pub fn to_unicode(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?.value(&mut cx);
    let (decoded, result) = domain_to_unicode(&input);
    match result {
        Ok(_) => Ok(cx.string(decoded)),
        Err(e) => cx.throw_error(format!("Error decoding string: {:?}", e)),
    }
}

fn en(mut cx: FunctionContext) -> JsResult<JsString> {
    let input: String = cx.argument::<JsString>(0)?.value(&mut cx);
    let char_vec: Vec<char> = input.chars().collect();
    match punycode::encode(&char_vec) {
        Some(encoded) => Ok(cx.string(encoded)),
        None => cx.throw_error("Error in Punycode encoding"),
    }
}

fn de(mut cx: FunctionContext) -> JsResult<JsString> {
    let input: String = cx.argument::<JsString>(0)?.value(&mut cx);
    match punycode::decode(&input) {
        Some(decoded_chars) => {
            let decoded_string: String = decoded_chars.into_iter().collect();
            Ok(cx.string(decoded_string))
        }
        None => cx.throw_error("Error in Punycode decoding"),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("toAscii", to_ascii)?;
    cx.export_function("toAsciiStrict", to_ascii_strict)?;
    cx.export_function("toUnicode", to_unicode)?;
    cx.export_function("encode", en)?;
    cx.export_function("decode", de)?;

    Ok(())
}
