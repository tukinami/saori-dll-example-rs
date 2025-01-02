//! デスクトップマスコット、「伺か」用DLLの一種、「SAORI」の例です。
//!
//! 拙作「[ukagaka-dll-macro](https://github.com/tukinami/ukagaka-dll-macro)」、
//! 「[saori-interface-rs](https://github.com/tukinami/saori-interface-rs)」の使用例でもあります。
//!
//! SAORIの機能としては、RGBの色の割合(0.0 ~ 1.0)から`sRGB`としてみたときの光度を返します。

use saori_interface_rs::*;
use ukagaka_dll_macro::*;

/// RGB(0.0 ~ 1.0)の文字列表記から、sRBGとしてみたときの光度を返す
///
/// 計算できない場合は`None`を返す
fn calc_lightness(args: &[String]) -> Option<f64> {
    if args.len() != 3 {
        return None;
    }

    args.iter()
        .map(|v| v.parse::<f64>().ok())
        .enumerate()
        .try_fold(0.0, |acc, (i, value)| {
            value.and_then(|v| match i {
                _ if !(0.0..=1.0).contains(&v) => None,
                0 => Some(v * 0.2126 + acc),
                1 => Some(v * 0.7152 + acc),
                2 => Some(v * 0.0722 + acc),
                _ => None,
            })
        })
}

/// SHIORIから来たリクエストを処理する
fn ukagaka_request(s: &[u8]) -> Vec<i8> {
    // リクエストの読み取り
    let request = match SaoriRequest::new(s) {
        Ok(v) => v,
        Err(_) => {
            // 読み取れなかった場合、Bad RequestとしてSHIORI側に返答
            return SaoriResponse::new_bad_request()
                .to_encoded_bytes()
                .unwrap_or(SaoriResponse::error_bytes());
        }
    };

    // リクエストから返答の雛形を作成
    let mut response = SaoriResponse::from_request(&request);

    // リクエストからのコマンドによって動作を変更
    match request.command() {
        SaoriCommand::GetVersion => {
            // バージョンを返す
            response.set_result(env!("CARGO_PKG_VERSION").to_string());
        }
        SaoriCommand::Execute => {
            // 主な処理
            // 処理結果がある場合とない(計算できなかった場合)で分岐
            if let Some(result) = calc_lightness(request.arguments()) {
                // 処理結果有の場合
                // Resultに結果を入れる
                response.set_result(format!("{}", result));
                // Value0で返答したい場合は以下:
                // response.set_value_at(0, format!("{}", result));
            } else {
                // 処理結果なしの場合
                // 引数が読み取れなかったのでBad RequestとしてSHIORI側に返答
                response.set_status(SaoriStatus::BadRequest);
            }
        }
    }

    // 返答を`Vec<i8>`にして返す
    // エンコードが失敗した場合、そのエラーをSHIORIに返答
    response
        .to_encoded_bytes()
        .unwrap_or(SaoriResponse::error_bytes())
}

// 関数`DllMain`を定義
define_dll_main!();
// 関数`unload`を定義
define_load!();
// 関数`request`を定義
define_request!(ukagaka_request);
// 関数`unload`を定義
define_unload!();

// 以下は`cargo test`コマンドで走るテスト用。
// 必要なければ削除
#[cfg(test)]
mod tests {
    use super::*;

    mod calc_lightness {
        use super::*;

        #[test]
        fn some_value_when_valid_args() {
            let case = ["0.5".to_string(), "0.5".to_string(), "0.5".to_string()];
            let result = calc_lightness(&case);
            assert_eq!(result, Some(0.5));

            let case = ["1.0".to_string(), "1".to_string(), "1.0".to_string()];
            let result = calc_lightness(&case);
            assert_eq!(result, Some(1.0));

            let case = ["0.0".to_string(), "0".to_string(), "0.0".to_string()];
            let result = calc_lightness(&case);
            assert_eq!(result, Some(0.0));
        }

        #[test]
        fn none_when_too_much_args() {
            let case = [
                "0.5".to_string(),
                "0.5".to_string(),
                "0.5".to_string(),
                "0.5".to_string(),
            ];
            let result = calc_lightness(&case);
            assert!(result.is_none());
        }

        #[test]
        fn none_when_less_args() {
            let case = ["0.5".to_string(), "0.5".to_string()];
            let result = calc_lightness(&case);
            assert!(result.is_none());
        }

        #[test]
        fn none_when_invalid_args_str() {
            let case = ["0.5".to_string(), "0.5".to_string(), "aaa".to_string()];
            let result = calc_lightness(&case);
            assert!(result.is_none());
        }

        #[test]
        fn none_when_invalid_args_small_number() {
            let case = ["0.5".to_string(), "0.5".to_string(), "-0.5".to_string()];
            let result = calc_lightness(&case);
            assert!(result.is_none());
        }

        #[test]
        fn none_when_invalid_args_large_number() {
            let case = ["0.5".to_string(), "1.5".to_string(), "0.5".to_string()];
            let result = calc_lightness(&case);
            assert!(result.is_none());
        }
    }

    mod ukagaka_request {
        use super::*;

        #[test]
        fn checking_value_when_valid_request_execute() {
            let case_raw = "EXECUTE SAORI/1.0\r\nSecurityLevel: Local\r\nArgument0: 0.5\r\nArgument1: 0.5\r\nArgument2: 0.5\r\nCharset: UTF-8\r\nSender: test\r\n\r\n\0";
            let case_bytes = case_raw.as_bytes();
            let result = ukagaka_request(&case_bytes);

            let expect_raw = "SAORI/1.0 200 OK\r\nCharset: UTF-8\r\nResult: 0.5\r\n\r\n\0";
            let expect_bytes: Vec<i8> = expect_raw.as_bytes().iter().map(|v| *v as i8).collect();
            assert_eq!(result, expect_bytes);
        }

        #[test]
        fn checking_value_when_valid_request_get_version() {
            let case_raw = "GET Version SAORI/1.0\r\nSecurityLevel: Local\r\nCharset: UTF-8\r\nSender: test\r\n\r\n\0";
            let case_bytes = case_raw.as_bytes();
            let result = ukagaka_request(&case_bytes);

            let expect_raw = concat!(
                "SAORI/1.0 200 OK\r\nCharset: UTF-8\r\nResult: ",
                env!("CARGO_PKG_VERSION"),
                "\r\n\r\n\0"
            );
            let expect_bytes: Vec<i8> = expect_raw.as_bytes().iter().map(|v| *v as i8).collect();
            assert_eq!(result, expect_bytes);
        }

        #[test]
        fn checking_value_when_invalid_argument() {
            let case_raw = "EXECUTE SAORI/1.0\r\nSecurityLevel: Local\r\nArgument0: aaa\r\nArgument1: 0.5\r\nArgument2: 0.5\r\nCharset: UTF-8\r\nSender: test\r\n\r\n\0";
            let case_bytes = case_raw.as_bytes();
            let result = ukagaka_request(&case_bytes);

            let expect_raw = "SAORI/1.0 400 Bad Request\r\nCharset: UTF-8\r\n\r\n\0";
            let expect_bytes: Vec<i8> = expect_raw.as_bytes().iter().map(|v| *v as i8).collect();
            assert_eq!(result, expect_bytes);
        }

        #[test]
        fn checking_value_when_invalid_request() {
            let case_raw = "SOMETHINGWRONG SAORI/1.0\r\nSecurityLevel: Local\r\nArgument0: 0.5\r\nArgument1: 0.5\r\nArgument2: 0.5\r\nCharset: UTF-8\r\nSender: test\r\n\r\n\0";
            let case_bytes = case_raw.as_bytes();
            let result = ukagaka_request(&case_bytes);

            let expect_raw = "SAORI/1.0 400 Bad Request\r\nCharset: UTF-8\r\n\r\n\0";
            let expect_bytes: Vec<i8> = expect_raw.as_bytes().iter().map(|v| *v as i8).collect();
            assert_eq!(result, expect_bytes);
        }
    }
}
