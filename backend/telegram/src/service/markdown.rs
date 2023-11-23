/*
   This module is used to parse common markdown text to telegram safe markdown
*/

use pulldown_cmark::{html, Event, Parser};
use reqwest::Url;

/*
MarkdownV2 style
To use this mode, pass MarkdownV2 in the parse_mode field. Use the following syntax in your message:

*bold \*text*
_italic \*text_
__underline__
~strikethrough~
||spoiler||
*bold _italic bold ~italic bold strikethrough ||italic bold strikethrough spoiler||~ __underline italic bold___ bold*
[inline URL](http://www.example.com/)
[inline mention of a user](tg://user?id=123456789)
![👍](tg://emoji?id=5368324170671202286)
`inline fixed-width code`
```
pre-formatted fixed-width code block
```
```python
pre-formatted fixed-width code block written in the Python programming language
```
Please note:

Any character with code between 1 and 126 inclusively can be escaped anywhere with a preceding '\' character, in which case it is treated as an ordinary character and not a part of the markup. This implies that '\' character usually must be escaped with a preceding '\' character.
Inside pre and code entities, all '`' and '\' characters must be escaped with a preceding '\' character.
Inside the (...) part of the inline link and custom emoji definition, all ')' and '\' must be escaped with a preceding '\' character.
In all other places characters '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!' must be escaped with the preceding character '\'.
In case of ambiguity between italic and underline entities __ is always greadily treated from left to right as beginning or end of underline entity, so instead of ___italic underline___ use ___italic underline_\r__, where \r is a character with code 13, which will be ignored.
A valid emoji must be provided as an alternative value for the custom emoji. The emoji will be shown instead of the custom emoji in places where a custom emoji cannot be displayed (e.g., system notifications) or if the message is forwarded by a non-premium user. It is recommended to use the emoji from the emoji field of the custom emoji sticker.
Custom emoji entities can only be used by bots that purchased additional usernames on Fragment.
https://core.telegram.org/bots/api#markdownv2-style
*/
pub fn cmark_to_telegram_v2(common_markdown: &str) -> String {
    let parser = Parser::new(common_markdown);

    let mut t_markdown_v2 = String::new();

    let parser = parser.map(|event| {
        match &event {
            Event::Text(text) => {
                // DEBUG EVENT: println!("Text: {:?}", text);
                if is_url(&text) {
                    // If it's a URL, we don't want to escape it
                    t_markdown_v2.push_str(&text);
                } else {
                    t_markdown_v2.push_str(&escape_telegram_markdown(&text));
                }
            }
            Event::Start(pulldown_cmark::Tag::Paragraph) => {
                // DEBUG EVENT: println!("Start Paragraph");
                // DO NOTHING! We'll end the paragraph with a double newline instead
            }
            Event::End(pulldown_cmark::Tag::Paragraph) => {
                // DEBUG EVENT: println!("End Paragraph");
                t_markdown_v2.push_str("\n\n");
            }
            Event::Start(pulldown_cmark::Tag::Emphasis) => {
                // DEBUG EVENT: println!("Start Emphasis");
                t_markdown_v2.push_str("_");
            }
            Event::End(pulldown_cmark::Tag::Emphasis) => {
                // DEBUG EVENT: println!("End Emphasis");
                t_markdown_v2.push_str("_");
            }
            Event::Start(pulldown_cmark::Tag::Strong) => {
                // DEBUG EVENT: println!("Start Strong");
                t_markdown_v2.push_str("*");
            }
            Event::End(pulldown_cmark::Tag::Strong) => {
                // DEBUG EVENT: println!("End Strong");
                t_markdown_v2.push_str("*");
            }
            Event::Start(pulldown_cmark::Tag::CodeBlock(kind)) => {
                // DEBUG EVENT: println!("Start Code Block");
                match kind {
                    pulldown_cmark::CodeBlockKind::Indented => {
                        // DEBUG EVENT: println!("Start Indented Code Block");
                        t_markdown_v2.push_str("```");
                    }
                    pulldown_cmark::CodeBlockKind::Fenced(language) => {
                        // DEBUG EVENT: println!("Start Fenced Code Block");
                        t_markdown_v2.push_str("```");
                        t_markdown_v2.push_str(&language);
                        t_markdown_v2.push_str("\n");
                    }
                }
            }
            Event::End(pulldown_cmark::Tag::CodeBlock(_kind)) => {
                // DEBUG EVENT: println!("End Code Block");
                t_markdown_v2.push_str("```");
            }
            Event::Start(pulldown_cmark::Tag::Link(_link_type, _url, _title)) => {
                // DEBUG EVENT: println!("Start Link");

                // The link title is sent as a text event in most cases, so we'll just open the markdown link here
                t_markdown_v2.push_str("[");
            }
            Event::End(pulldown_cmark::Tag::Link(_link_type, url, _title)) => {
                // DEBUG EVENT: println!("End Link");

                // Inline and other Links send their title as a text event, so we just close out the markdown link here, and use the URL ...
                t_markdown_v2.push_str("](");
                t_markdown_v2.push_str(&url);
                t_markdown_v2.push_str(")");
            }
            Event::Code(text) => {
                // DEBUG EVENT: println!("Code: {:?}", text);
                // An inline code node - https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.Event.html
                t_markdown_v2.push_str("`");
                t_markdown_v2.push_str(&text);
                t_markdown_v2.push_str("`");
            }
            Event::Html(text) => {
                // DEBUG EVENT: println!("HTML: {:?}", text);

                // It's not clear what to do with this, but it's not supported by telegram markdown
                // We'll put it in a code block?
                t_markdown_v2.push_str("```");
                t_markdown_v2.push_str(&escape_telegram_markdown(&text)); // Not sure if it's best escape this or not, playing it safe...
                t_markdown_v2.push_str("```");
            }
            Event::FootnoteReference(text) => {
                t_markdown_v2.push_str(&escape_telegram_markdown(&text));
            }
            Event::SoftBreak => {
                t_markdown_v2.push_str("\n");
            }
            Event::HardBreak => {
                t_markdown_v2.push_str("\n\n");
            }
            Event::Rule => {
                t_markdown_v2.push_str("\n-----\n");
            }
            Event::Start(pulldown_cmark::Tag::Heading(_, _, _)) => {
                // DEBUG EVENT: println!("Start Heading");
                // Telegram doesn't support headings, so we'll just do Bold Underline
                t_markdown_v2.push_str("*__");
            }
            Event::End(pulldown_cmark::Tag::Heading(_, _, _)) => {
                // DEBUG EVENT: println!("End Heading");
                // Telegram doesn't support headings, so we'll just do Bold Underline
                t_markdown_v2.push_str("__*\n");
            }
            Event::Start(_tag) => {
                // Telegram doesn't support this, and we're not going to bother with it
                log::error!("Unknown Tag: {:?} in Telegram Markdown Parser", _tag);
            }
            Event::End(_tag) => {
                // Telegram doesn't support this, and we're not going to bother with it
                log::error!("Unknown End Tag: {:?} in Telegram Markdown Parser", _tag);
            }
            _ => {
                log::error!("Unknown Event: {:?} in Telegram Markdown Parser", event);
                // Telegram doesn't support some of these, so we'll just do nothing, and hope some text appears
            }
        };
        event
    });

    // Process the events but creating an html output...
    // This is needed to trigger the parser, there's probably a better way, but this works for now
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    return t_markdown_v2
        .strip_suffix("\n\n")
        .unwrap_or(&t_markdown_v2)
        .to_string();
}

// In all other places characters '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!' must be escaped with the preceding character '\'.
// https://core.telegram.org/bots/api#markdownv2-style
fn escape_telegram_markdown(text: &str) -> String {
    let mut escaped_text = String::new();
    for c in text.chars() {
        match c {
            '_' | '*' | '[' | ']' | '(' | ')' | '~' | '>' | '#' | '+' | '-' | '=' | '|' | '{'
            | '`' | '}' | '.' | '!' | '\\' => {
                escaped_text.push('\\');
                escaped_text.push(c);
            }
            _ => escaped_text.push(c),
        }
    }
    escaped_text
}

// Function to determine if a string is a valid URL
// Used to determine if we should escape a string or not.
// Telegram says to escape characters in a URL, but we don't want to do that if it's a URL
fn is_url(text: &str) -> bool {
    let result = Url::parse(text);
    result.is_ok()
}

#[cfg(test)]
mod test {
    use super::*;
    // Test basic Bold
    #[test]
    fn test_cmark_telegram_v2_bold() {
        let cmarkdown = "**bold * text**";
        let expected = "*bold \\* text*"; // Single star to say it's bold, and escaped star in the middle
        let result = cmark_to_telegram_v2(cmarkdown);
        assert_eq!(result, expected);
    }

    // Test basic Italic
    #[test]
    fn test_cmark_telegram_v2_italic() {
        let cmarkdown = "_italic * text_";
        let expected = "_italic \\* text_"; // Single underscore to say it's emphasis, and escaped star in the middle
        let result = cmark_to_telegram_v2(cmarkdown);
        assert_eq!(result, expected);
    }

    //~strikethrough~
    #[test]
    fn test_cmark_telegram_v2_strikethrough() {
        let cmarkdown = "~strikethrough~";
        // NOTE: This isn't ideal, would be good if it would show as strike through in telegram and not escape it. Could improve in future?
        let expected = "\\~strikethrough\\~"; // Single star to say it's bold, and escaped star in the middle
        let result = cmark_to_telegram_v2(cmarkdown);
        assert_eq!(result, expected);
    }

    // ||spoiler||
    #[test]
    fn test_cmark_telegram_v2_spoiler() {
        let cmarkdown = "||spoiler||";
        // NOTE: This isn't ideal, would be good if it would show as spoiler in telegram and not escape it. Could improve in future?
        let expected = "\\|\\|spoiler\\|\\|"; // Single star to say it's bold, and escaped star in the middle
        let result = cmark_to_telegram_v2(cmarkdown);
        assert_eq!(result, expected);
    }

    // Bold, Italic
    #[test]
    fn test_cmark_telegram_v2_bold_italic() {
        let cmarkdown = "**bold _italic_ bold**";
        let expected = "*bold _italic_ bold*";
        let result = cmark_to_telegram_v2(cmarkdown);
        assert_eq!(result, expected);
    }

    // Include a link
    #[test]
    fn test_cmark_telegram_v2_links() {
        let cmarkdown = "[inline URL](http://www.example.com/)";
        let expected = "[inline URL](http://www.example.com/)";
        let result = cmark_to_telegram_v2(cmarkdown);
        assert_eq!(result, expected);

        // Auto Link
        let cmarkdown = "<http://www.example.com/>";
        let expected = "[http://www.example.com/](http://www.example.com/)";
        let result = cmark_to_telegram_v2(cmarkdown);
        assert_eq!(result, expected);

        // No Link
        let cmarkdown = "http://www.example.com/";
        let expected = "http://www.example.com/";
        let result = cmark_to_telegram_v2(cmarkdown);
        assert_eq!(result, expected);
    }

    // Pre-formatted fixed-width code block
    #[test]
    fn test_cmark_telegram_v2_code() {
        let cmarkdown = "`inline code`";
        let expected = "`inline code`";
        let result = cmark_to_telegram_v2(cmarkdown);
        assert_eq!(result, expected);

        // Multiline Code
        let cmarkdown = r#"```
Line1
Line2
Line3
```"#; // Line needs to start with ``` for the parser to pick it up (aka don't indent this!)
        let expected = "```\nLine1\nLine2\nLine3\n```";
        let result = cmark_to_telegram_v2(cmarkdown);
        assert_eq!(result, expected);
    }

    // Cold Chain Example
    #[test]
    fn test_cmark_telegram_v2_cc() {
        // example cold chain markdown
        let cmarkdown = r#"**Sensor is now ok!**

**Facility**: Facility
**Location**: location_name
**Sensor**: sensor_name

**Date**:  22 Feb 2021
**Time**:  16:20
**Temperature**: 5.10°C
**Last data received**: 2 hours ago
"#; // Don't indent this!
        let expected = r#"*Sensor is now ok\!*

*Facility*: Facility
*Location*: location\_name
*Sensor*: sensor\_name

*Date*:  22 Feb 2021
*Time*:  16:20
*Temperature*: 5\.10°C
*Last data received*: 2 hours ago"#; // Don't indent this!
        let result = cmark_to_telegram_v2(cmarkdown);
        assert_eq!(result, expected);
    }

    // Test emoji!
    #[test]
    fn test_cmark_telegram_v2_emoji() {
        let cmarkdown = "👍";
        let expected = "👍";
        let result = cmark_to_telegram_v2(cmarkdown);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_escape_telegram_markdown() {
        let text = "This is a test of markdown - escaping";
        let escaped_text = escape_telegram_markdown(text);
        assert_eq!(escaped_text, "This is a test of markdown \\- escaping");
    }
}
