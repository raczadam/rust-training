use jni::objects::JString;
use jni::sys::jboolean;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_org_example_App_palindrome(
    env: JNIEnv,
    _class: jni::objects::JClass,
    input: JString,
) -> jboolean {
    let input: String = match env.get_string(input) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };

    if palindrome(input) {
        return 1;
    }
    return 0;
}

fn palindrome(input: String) -> bool {
    if input.len() == 0 {
        return true;
    }
    let mut last = input.len() - 1;
    let mut first = 0;

    let my_vec = input.as_bytes();

    while first < last {
        if my_vec[first] != my_vec[last] {
            return false;
        }
        first += 1;
        last -= 1;
    }
    return true;
}
