// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    app::{AppExit, ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::schedule::ReportExecutionOrderAmbiguities,
    log::LogPlugin,
    prelude::*,
    utils::Duration,
};
use rand::random;

///
/// # ECSガイド
///
/// ## ECSシステムの利点
///
/// データ指向：機能はデータによって駆動されます
/// クリーンなアーキテクチャ：機能の結合が緩い/深くネストされた継承を防ぐ
/// 高性能：超並列でキャッシュに対応
///
/// ## ECSの定義
///
/// - コンポーネント：通常のRustデータ型です。 通常、単一の機能にスコープされます  
///     例：位置、速度、健康、色、名前
///
/// - エンティティ：一意のIDを持つコンポーネントのコレクション
///     例: Entity1 { Name("Alice"), Position(0, 0) }, Entity2 { Name("Bill"), Position(10, 5)
///
/// - リソース：共有されたグローバルなデータ  
///     例：アセットストレージ、イベント、システム状態
///
/// - システム：エンティティ、コンポーネント、およびリソースでロジックを実行します
///     例：移動システム、ダメージシステム
///
/// ECSについて少し知ったところで、Bevyコードを見てみましょう。
/// ここで、BevyのECSが実際にどのように見えるかを説明するための簡単な「ゲーム」を作成します。
///

//
// コンポーネント：エンティティに追加する機能の一部。 これらは通常のRustデータ型です
//

// 私たちのゲームには多くの「プレイヤー」がいます。 各プレイヤーには、彼らを識別する名前があります
#[derive(Component)]
struct Player {
    name: String,
}

// 各プレイヤーにもスコアがあります。 このコンポーネントはそのスコアを保持します
#[derive(Component)]
struct Score {
    value: usize,
}

//
// リソース：システムがアクセスできる「グローバル」状態。 これらも通常のRustデータ型です。
//

// このリソースは、ゲームに関する情報を保持しています
#[derive(Default)]
struct GameState {
    current_round: usize,
    total_players: usize,
    winning_player: Option<String>,
}

// このリソースは、私たちの「ゲーム」のルールを提供します。
struct GameRules {
    winning_score: usize,
    max_rounds: usize,
    max_players: usize,
}

//
// システム：エンティティ、コンポーネント、およびリソースで実行されるロジック。
// これらは通常、アプリが更新されるたびに1回実行されます
//

// これは最も単純なタイプのシステムです。実行毎に「このゲームは楽しい！」と印刷するだけです。
fn print_message_system() {
    println!("This game is fun!");
}

// システムは、リソースを読み取って変更することもできます。
// このシステムは、更新のたびに新しい「ラウンド」を開始します
// Res<GameRules>は読み取り専用です。 ResMut<GameState>はリソースを変更できます
fn new_round_system(game_rules: Res<GameRules>, mut game_state: ResMut<GameState>) {
    game_state.current_round += 1;
    println!(
        "Begin round {} of {}",
        game_state.current_round, game_rules.max_rounds
    );
}

// このシステムは、「プレーヤー」および「スコア」コンポーネントを使用して、
// 各エンティティのスコアを更新します。
fn score_system(mut query: Query<(&Player, &mut Score)>) {
    for (player, mut score) in query.iter_mut() {
        let scored_a_point = random::<bool>();
        if scored_a_point {
            score.value += 1;
            println!(
                "{} scored a point! Their score is: {}",
                player.name, score.value
            );
        } else {
            println!(
                "{} did not score a point! Their score is: {}",
                player.name, score.value
            );
        }
    }
}

// このシステムは、「Player」および「Score」コンポーネントを持つ
// すべてのエンティティで実行されますが、「GameRules」リソースにアクセスして、
// プレーヤーが勝ったかどうかを判断します。
fn score_check_system(
    game_rules: Res<GameRules>,
    mut game_state: ResMut<GameState>,
    query: Query<(&Player, &Score)>,
) {
    for (player, score) in query.iter() {
        if score.value == game_rules.winning_score {
            game_state.winning_player = Some(player.name.clone());
        }
    }
}

// このシステムは、適切な条件を満たせばゲームを終了します。
// これにより、AppExitイベントが発生し、アプリに終了するように指示します。
// イベントの使用について詳しく知りたい場合は、「event.rs」の例を確認してください。
fn game_over_system(
    game_rules: Res<GameRules>,
    game_state: Res<GameState>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if let Some(ref player) = game_state.winning_player {
        println!("{} won the game!", player);
        app_exit_events.send(AppExit);
    } else if game_state.current_round == game_rules.max_rounds {
        println!("Ran out of rounds. Nobody wins!");
        app_exit_events.send(AppExit);
    }

    println!();
}

// これは、アプリの起動時に1回だけ実行される「スタートアップ」システムです。
// スタートアップシステムは通常、ゲームの初期の「状態」を作成するために使用されます。
// 「スタートアップ」システムと「通常の」システムを区別する唯一のことは、
// それがどのように登録されるかです。
//
// StartUp: app.add_startup_system(startup_system)
// Normal:  app.add_system(normal_system)
//
fn startup_system(mut commands: Commands, mut game_state: ResMut<GameState>) {
    // ゲームルールリソースを作成する
    commands.insert_resource(GameRules {
        max_rounds: 10,
        winning_score: 4,
        max_players: 4,
    });

    // 私たちの世界に何人かのプレーヤーを追加します。
    // プレイヤーはスコア0から始めます...私たちはゲームが公平であることを望んでいます！
    commands.spawn_batch(vec![
        (
            Player {
                name: "Alice".to_string(),
            },
            Score { value: 0 },
        ),
        (
            Player {
                name: "Bob".to_string(),
            },
            Score { value: 0 },
        ),
    ]);

    // プレーヤーの総数を「2」に設定します
    game_state.total_players = 2;
}

// このシステムは、コマンドバッファーを使用して、反復ごとに（潜在的に）新しいプレーヤーをゲームに追加します。
// 通常のシステムは、並行して実行されるため、Worldインスタンスに直接安全にアクセスすることはできません。
// 私たちの世界にはすべてのコンポーネントが含まれているため、その任意の部分を並列に変更することはスレッドセーフではありません。
// コマンドバッファを使用すると、ワールドに直接アクセスせずに、 ワールドへの変更をキューに入れることができます。
fn new_player_system(
    mut commands: Commands,
    game_rules: Res<GameRules>,
    mut game_state: ResMut<GameState>,
) {
    // 新しいプレーヤーをランダムに追加します
    let add_new_player = random::<bool>();
    if add_new_player && game_state.total_players < game_rules.max_players {
        game_state.total_players += 1;
        commands.spawn_bundle((
            Player {
                name: format!("Player {}", game_state.total_players),
            },
            Score { value: 0 },
        ));

        println!("Player {} joined the game!", game_state.total_players);
    }
}

// ワールドまたはリソースへの完全な即時の読み取り/書き込みアクセスが本当に必要な場合は、
// 「専用システム」を使用できます。
// WARNING：これらは、終了するまで他のシステムのすべての並列実行をブロックするため、
// パフォーマンスを気にする場合は、通常は避ける必要があります。
//
#[allow(dead_code)]
fn exclusive_player_system(world: &mut World) {
    // this does the same thing as "new_player_system"
    let total_players = world.get_resource_mut::<GameState>().unwrap().total_players;
    let should_add_player = {
        let game_rules = world.get_resource::<GameRules>();
        let add_new_player = random::<bool>();
        add_new_player && total_players < game_rules.unwrap().max_players
    };
    // Randomly add a new player
    if should_add_player {
        world.spawn().insert_bundle((
            Player {
                name: format!("Player {}", total_players),
            },
            Score { value: 0 },
        ));

        // let mut game_state = world.get_resource_mut::<GameState>();
        // game_state.unwrap().total_players += 1;
    }
}

// システムには、独自の「ローカル」状態が必要な場合があります。
// BevyのECSは、この場合にLocal<T>リソースを提供します。
// Local <T>リソースはシステムに固有であり、ユーザーに代わって自動的に初期化されます（まだ存在しない場合）。
// システムのIDがある場合は、`Resources::get_local()`を使用してResourcesコレクション内の
// ローカルリソースに直接アクセスすることもできます。一般に、この機能が必要になるのは次の場合のみです。
//
// 1: 同じシステムの複数のインスタンスがあり、それぞれに固有の状態が必要です。
// 2: 現在のシステムで上書きしたくないリソースのグローバルバージョンがすでにあります。
// 3: システムのリソースをグローバルリソースとして登録するのが面倒です
//

//
// これは私たちのゲームに関連することは何もしません、それは説明の目的のためにここにあります
//
// #[allow(dead_code)]
// fn local_state_system(mut state: Local<State>, query: Query<(&Player, &Score)>) {
//     for (player, score) in query.iter() {
//         println!("processed: {} {}", player.name, score.value);
//     }
//     println!("this system ran {} times", state.counter);
//     state.counter += 1;
// }

#[derive(Default)]
struct State {
    _counter: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum MyStage {
    BeforeRound,
    AfterRound,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum MyLabels {
    ScoreCheck,
}

// Bevyアプリのエントリーポイント
fn main() {
    // Bevyアプリは、ビルダーパターンを使用して作成されます。
    // ビルダーを使用して、システム、リソース、プラグインをアプリに追加します
    App::new()
        // このようにリソースをアプリに追加できます
        .insert_resource(State { _counter: 0 })
        // 一部のシステムは、設定をリソースとして追加することによって構成されます一部のシステムは、
        // 設定をリソースとして追加することによって構成されます
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs(5)))
        // プラグインは、アプリビルダー呼び出しのグループ化されたセットです（ここで行っているように）。
        // ゲームをプラグインに簡単に変えることができますが、そのプラグインの例を確認できます
        // 以下のプラグインは、アプリの「システムスケジュール」を5秒に1回実行します（上記で構成）。
        .add_plugin(ScheduleRunnerPlugin::default())
        // DefaultまたはFromWorldトレイトを実装するリソースは、次のように追加できます。
        .init_resource::<GameState>()
        // 起動システムは、他のすべてのシステムの前に1回だけ実行されます。
        // これらは通常、アプリの初期化コードに使用されます（例：エンティティとリソースの追加）
        .add_startup_system(startup_system)
        // my_system呼び出しは、通常のrust関数をECSシステムに変換します。
        .add_system(print_message_system)
        //
        // システム実行順序
        //
        // 各システムは「ステージ」に属しており、各ティック内のシステムの実行戦略と幅広い順序を制御します。
        // スタートアップステージ（スタートアップシステムが登録されている）は、通常のステージが始まる前に常に完了し、
        // ステージ内のすべてのシステムは、次のステージが進む前に完了する必要があります。
        // すべてのステージが終了すると、メインループが完了し、再開します。
        //
        // デフォルトでは、データへの可変アクセスが必要な場合を除いて、すべてのシステムが並行して実行されます。
        // これは効率的ですが、順序が重要になる場合があります。
        // たとえば、「ゲームオーバー」システムを他のすべてのシステムの後に実行して、
        // ゲームを誤って余分なラウンドで実行しないようにします。
        //
        // 各システムを別々のステージに分割するのではなく、関連するシステムに
        // `.label`のラベルを付けてから、`.before`または`.after`メソッドを使用して、
        // システム間で明示的な順序付けを強制する必要があります。
        // システムは、「順序依存関係」を持つすべてのシステムが完了するまでスケジュールされません。
        //
        // これを行うと、ほぼすべての場合で、システムをステージ間で分割する場合に比べてパフォーマンスが向上します。
        // これは、スケジューリングアルゴリズムにより、システムを並列で実行する機会が増えるためです。
        // ただし、ステージは引き続き必要です。ステージの終わりは、
        // システムによって発行された「コマンド」が処理されるハード同期ポイント（つまり、システムが実行されていない）です。
        // これが必要なのは、コマンドが、エンティティの生成や削除、リソースの追加や削除など、
        // システムの稼働と互換性のない操作を実行する可能性があるためです。
        //
        // add_system（system）は、デフォルトでUPDATEステージにシステムを追加します。
        // ただし、必要に応じて、ステージを手動で指定できます。
        // 以下はadd_system（score_system）と同等です
        .add_system_to_stage(CoreStage::Update, score_system)
        // 新しいステージを作成することもできます。 これが私たちのゲームステージの順序がどのようになるかです
        // "before_round": new_player_system, new_round_system
        // "update": print_message_system, score_system
        // "after_round": score_check_system, game_over_system
        .add_stage_before(
            CoreStage::Update,
            MyStage::BeforeRound,
            SystemStage::parallel(),
        )
        .add_stage_after(
            CoreStage::Update,
            MyStage::AfterRound,
            SystemStage::parallel(),
        )
        .add_system_to_stage(MyStage::BeforeRound, new_round_system)
        .add_system_to_stage(MyStage::BeforeRound, new_player_system)
        // 引数として`＆mut World`をとるシステムは、` .exclusive_system（）`を呼び出さなければなりません。
        // note: 以下はコンパイルされません。
        .add_system_to_stage(
            MyStage::BeforeRound,
            exclusive_player_system.exclusive_system(),
        )
        //
        // 明示的な順序付けを使用して、game_overシステムがscore_check_systemの後に実行されるようにすることができます
        // 最初に、参照するシステムに`.label`を使用してラベルを付けます。
        // 次に、`.before`または`.after`のいずれかを使用して、関係が必要な順序を記述します。
        .add_system_to_stage(
            MyStage::AfterRound,
            score_check_system.label(MyLabels::ScoreCheck),
        )
        .add_system_to_stage(
            MyStage::AfterRound,
            game_over_system.after(MyLabels::ScoreCheck),
        )
        // `LogPlugin`を使用してコンソールで生成された出力を調べ、次のリソースをアプリに追加することで、
        // 実行順序のあいまいさをシステムで確認できます
        // このチェッカーによって報告されるすべてが潜在的な問題であるとは限らないことに注意してください。
        // その判断は自分で行う必要があります。
        .add_plugin(LogPlugin::default())
        .insert_resource(ReportExecutionOrderAmbiguities)
        // このrun（）の呼び出しにより、作成したアプリが起動します。
        .run()
}
