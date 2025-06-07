*[English README.md available affter Japanese]*

# `woka` - "Work at"

複雑な SSH 接続をある程度やりやすくする SSH コマンドのラッパーです!

### 動機
こういうことがあったためです:

- **同じマシンに別のネットワークから接続したい** — 異なるネットワークで同じホストにアクセスしようとするとき、
  各ネットワーク用の設定を `.ssh/config` に書いて、ネットワークごとに専用のホスト名で SSH を実行する必要がありました。
  ただ、これだとコマンドを叩くときに毎回ネットワークを意識する必要があったので、複数の設定を自動試行できるようにしたいという動機がありました。
- **ポートフォワード指定を短くやりたい** — 複数のプロジェクトで稼働するとき、それぞれ用のポートフォワードの設定をする必要がありました。
  SSH コマンドのポートフォワード指定はそれなりに長いので、もうちょっと楽に書きたいという動機がありました。

### これいる?

個人のワークフローをアプリケーションに落とし込んだ…… くらいの感じなので、ゆーて万人受けは狙ってないです。私が欲しいと思った機能を増やしていきます!
とはいえ、気になって見に来てくれたり、使ってくださったり、Contribution してくださる方がいれば大変うれしいです!

### 使い方

Todo...

### なんで `woka`?

後で `woko` ("Work On") という、プロジェクトごとの環境を整えるのに使える、似たようなツールを作ろうとしています!
`woka` と `woko` を併用すると "Wokawoko" という楽しげな響きになるし、名前の由来もわかりやすくて覚えやすいのでこの名前にしました。

---

# `woka` - "Work At"

A SSH command wrapper to make a complicated connection somewhat handy!

### Motivation

- **I wanted to access to the same machine from different network** -- 
  To do that, I need to write a several configuration for each networks to `.ssh/config`, and
  run `ssh` with the hostname that works for the currently connecting network. But this makes me
  necessary to think about the network I'm in whenever I `ssh` and that's annoying, so I wanted a tool
  that can auto-try several configs for me.

- **I wanted to make port forwarding arguments (`-L` / `-R`) shorter** --
  When I work on several projects, I wanted to pass `-L` / `-R` distinctly for each project.
  SSH's `-L` / `-R` option syntax is somewhat long (especially I really rarely forward non-localhost port),
  so I wanted to pass the option in more shorter syntax.

### Do I need this?

Probably not; this is an application that represented my workflow into Rust application,
and this application is not meant to be made to catch people's eyes. And I'm going to implement the feature I'd like!
That said I'd really, really appreciate if you came to see, use, or contribute!

### How it works

Todo...

### Why the name `woka`?

I'm planning to make another tool similiar to this, named woko (Work On), to help preparing the working environment for the each project!
If I use woka and woko together, it'd be able to say "Wokawoko" stack (?) and it sounds funny, so I decided to use this name.
As a plus the name turned out to be very easy to remember, so it's better!
