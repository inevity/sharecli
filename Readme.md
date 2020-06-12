## Sharecli ghost example
```
cargo build && sharecli ghost list post -q 'fields=title,url,id,slug,status&limit=all&status=draft&page=2'|grep test
cargo build && ./target/debug/sharecli ghost list post -q 'fields=title,url,id,slug&include=tags,authors&limit=2&filter=status:published&page=1'
cargo build && ./target/debug/sharecli ghost list post -q 'fields=title,url,id,slug,status,excerpt,custom_excerpt&include=tags,authors&limit=1&filter=status:published&page=1'
cargo build && ./target/debug/sharecli ghost list post -q 'fields=title,url,id,slug,status,excerpt,custom_excerpt,updated_at&include=tags,authors&limit=1&filter=status:published&page=1'
sharecli ghost delete post --id 5e73458d4cea2827f8cf4b96,5e7344774cea2827f8cf4b92
sharecli ghost delete post --id 5e7343c64cea2827f8cf4b8e,5e7343654cea2827f8cf4b8a,5e73430b4cea2827f8cf4b86,5e7341644cea2827f8cf4b7e
cargo build && ./target/debug/sharecli ghost list post -q 'fields=title,url,id,slug,status,excerpt,custom_excerpt,updated_at&include=tags,authors&limit=400&filter=status:draft&page=1'|grep trade
cargo build && ./target/debug/sharecli ghost edit post -d '{"md": "tradeidea.md","custom_excerpt":"trade", "id":"5e96b5544cea2827f8cf6c3b","updated_at": "2020-04-15T07:18:44.000Z"}'
cargo build && ./target/debug/sharecli ghost edit post -d '{"md": "posts/tproxy.md", "id":"5eabcc164cea2827f8cf81b1", "updated_at": "2020-05-01T14:11:28.000Z"}'
cargo build && ./target/debug/sharecli ghost add post -d '{"md": "./a.md", "tags": [], "status": "draft", "custom_excerpt":"" }'
       md must path, tags can empty, status can draft, custom_excerpt must.
cargo build && ./target/debug/sharecli ghost add post -d '{"md": "./a.md", "custom_excerpt":"" }'
cargo build && ./target/debug/sharecli ghost add post -d '{"md": "test.md", "title":"test post2","custom_excerpt":"abstract"}' 
  must other tags[], authors =[],

cargo build && ./target/debug/sharecli ghost edit post -d '{"md": "./posts/targethungissuesummary.md","tags": ["Linux Kernle"], "id":"5eccc39e4cea2827f8cf82ae", "updated_at": "2020-05-26T07:22:06.000Z"}'
cargo build && ./target/debug/sharecli ghost list post -q 'fields=title,url,id,slug,status,excerpt,custom_excerpt,updated_at&include=tags,authors&limit=400&filter=status:draft&page=1'|grep revist
cargo build && ./target/debug/sharecli ghost edit post -d '{"md": "./posts/targethungissuesummary.md", "id":"5eccc39e4cea2827f8cf82ae", "updated_at": "2020-05-26T11:00:49.000Z"}'
```

## Done
* ghost blog platform

## Todo
* code block need indent
* upload image to ghost ,then no need to fix in the admin gui of ghost
* kejianjikede editor ,no need config the format 
* weibo
* weixin
* toutiao
* twitter
