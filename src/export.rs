use crate::stats::Stats;

const PLOTLY_URL: &str = "https://cdn.plot.ly/plotly-3.0.1.min.js";

fn json_string(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            // Keeps the data from closing the script element or opening a comment.
            '<' | '>' | '&' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_array<T>(items: impl IntoIterator<Item = T>, f: impl Fn(T) -> String) -> String {
    format!(
        "[{}]",
        items.into_iter().map(f).collect::<Vec<_>>().join(",")
    )
}

fn number(value: f64) -> String {
    if value.is_finite() {
        format!("{value:.4}")
    } else {
        "null".into()
    }
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Builds a self-contained report; `stats[tokenizer][language]`.
pub fn build_report(labels: &[String], languages: &[&str], stats: &[&Vec<Stats>]) -> String {
    let metric = |f: fn(&Stats) -> f64| {
        json_array(stats.iter(), |per_language| {
            json_array(per_language.iter(), |s| number(f(s)))
        })
    };
    let data = format!(
        "const languages={};const tokenizers={};const metrics={{\
         chars:{},bytes:{},words:{},tokens:{},charsPerToken:{},bytesPerToken:{},tokensPerWord:{}}};",
        json_array(languages, |l| json_string(l)),
        json_array(labels, |l| json_string(l)),
        metric(|s| s.chars as f64),
        metric(|s| s.bytes as f64),
        metric(|s| s.words as f64),
        metric(|s| s.tokens as f64),
        metric(Stats::chars_per_token),
        metric(Stats::bytes_per_token),
        metric(Stats::tokens_per_word),
    );

    let mut rows = String::new();
    for (language, name) in languages.iter().enumerate() {
        for (tokenizer, label) in labels.iter().enumerate() {
            let s = &stats[tokenizer][language];
            rows.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{:.2}</td><td>{:.2}</td><td>{:.2}</td></tr>\n",
                html_escape(name),
                html_escape(label),
                s.chars,
                s.bytes,
                s.words,
                s.tokens,
                s.chars_per_token(),
                s.bytes_per_token(),
                s.tokens_per_word(),
            ));
        }
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Tokenizer statistics</title>
<script src="{PLOTLY_URL}" charset="utf-8"></script>
<style>
body{{font-family:system-ui,sans-serif;margin:24px auto;max-width:1200px;padding:0 16px}}
.chart{{height:460px;margin-bottom:32px}}
table{{border-collapse:collapse;width:100%;font-size:14px}}
th,td{{border:1px solid #ccc;padding:4px 8px;text-align:right}}
th:nth-child(-n+2),td:nth-child(-n+2){{text-align:left}}
</style>
</head>
<body>
<h1>Tokenizer statistics</h1>
<p>Characters exclude whitespace; words are whitespace-separated (Chinese and Japanese kana count per character). Charts load Plotly from {PLOTLY_URL}, so an internet connection is required to view them.</p>
<h2>Compression efficiency by language</h2>
<div id="chars" class="chart"></div>
<div id="bytes" class="chart"></div>
<div id="words" class="chart"></div>
<h2>Tokenizer comparison</h2>
<div id="average" class="chart"></div>
<div id="tokens" class="chart"></div>
<div id="relative" class="chart"></div>
<h2>Matrix</h2>
<div id="heatmap" class="chart"></div>
<h2>Data</h2>
<table>
<tr><th>Language</th><th>Tokenizer</th><th>Characters</th><th>UTF-8 bytes</th><th>Words</th><th>Tokens</th><th>Chars/token</th><th>Bytes/token</th><th>Tokens/word</th></tr>
{rows}</table>
<script>
{data}
const bars=(id,title,values,yTitle)=>Plotly.newPlot(id,tokenizers.map((name,i)=>({{type:'bar',name,x:languages,y:values[i]}})),{{title,barmode:'group',yaxis:{{title:yTitle}}}},{{responsive:true}});
bars('chars','Characters per token (higher = better compression)',metrics.charsPerToken,'characters/token');
bars('bytes','UTF-8 bytes per token (higher = better compression)',metrics.bytesPerToken,'bytes/token');
bars('words','Tokens per word (lower = better compression)',metrics.tokensPerWord,'tokens/word');
bars('tokens','Total tokens per text',metrics.tokens,'tokens');
const mean=a=>a.reduce((s,v)=>s+v,0)/a.length;
Plotly.newPlot('average',[{{type:'bar',x:tokenizers,y:metrics.charsPerToken.map(mean)}}],{{title:'Average characters per token across all languages',yaxis:{{title:'characters/token'}}}},{{responsive:true}});
Plotly.newPlot('relative',tokenizers.map((name,i)=>({{type:'bar',name,x:languages,y:metrics.tokens[i].map((v,l)=>100*v/metrics.tokens[0][l])}})),{{title:'Token count relative to '+tokenizers[0]+' (%, lower = fewer tokens)',barmode:'group',yaxis:{{title:'%'}}}},{{responsive:true}});
Plotly.newPlot('heatmap',[{{type:'heatmap',x:tokenizers,y:languages,z:languages.map((_,l)=>tokenizers.map((_,t)=>metrics.charsPerToken[t][l])),colorscale:'RdYlGn',colorbar:{{title:'chars/token'}}}}],{{title:'Characters per token: languages × tokenizers',yaxis:{{autorange:'reversed'}}}},{{responsive:true}});
</script>
</body>
</html>
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_escapes_untrusted_names_and_contains_data() {
        let stats = vec![Stats {
            chars: 10,
            bytes: 20,
            words: 2,
            tokens: 5,
            sample: String::new(),
        }];
        let labels = vec!["</script><b>\"x\"".to_string()];
        let html = build_report(&labels, &["Русский"], &[&stats]);
        assert!(!html.contains("</script><b>"));
        assert!(html.contains("\\u003c/script\\u003e"));
        assert!(html.contains("&lt;/script&gt;"));
        assert!(html.contains("charsPerToken:[[2.0000]]"));
    }
}
