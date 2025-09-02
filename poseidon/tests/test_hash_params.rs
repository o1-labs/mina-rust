use mina_curves::pasta::Fp;
use poseidon::hash::params::*;
use std::str::FromStr;

// Helper function to create Fp from decimal string
fn fp_from_str(s: &str) -> Fp {
    Fp::from_str(s).expect("Failed to parse field element")
}

// Regression tests for all hash parameters to ensure their initial states remain consistent
// These values were captured from the actual computation and serve as regression tests

#[test]
fn test_mina_account() {
    let state = MINA_ACCOUNT.state();
    let expected = [
        fp_from_str(
            "21547009634669789644192675386133007766042650219024716227935570378579547706642",
        ),
        fp_from_str("3869977418072959680344087467966431440327948593054069717779845397512987438978"),
        fp_from_str(
            "17591003611016737523041467644989399067682599282318802410210271366172380277153",
        ),
    ];
    assert_eq!(state, expected, "MINA_ACCOUNT state changed");
}

#[test]
fn test_mina_proto_state() {
    let state = MINA_PROTO_STATE.state();
    let expected = [
        fp_from_str("5218970939948495870036503265499543025475317910763049867270287867667146978870"),
        fp_from_str("7663210626148314949787033187186036425676070286961909238040356477815169631084"),
        fp_from_str(
            "19859188289320816036969227839574854326171440874550138016648548415357198703337",
        ),
    ];
    assert_eq!(state, expected, "MINA_PROTO_STATE state changed");
}

#[test]
fn test_mina_proto_state_body() {
    let state = MINA_PROTO_STATE_BODY.state();
    let expected = [
        fp_from_str("3548547909990922956559515810876765435326873020883079662683136168632773655275"),
        fp_from_str("134182536761489093478066959027928272525080293912190881939140820794450385287"),
        fp_from_str(
            "18910449726094816833941350890285540874861148441082116020102338532207375519343",
        ),
    ];
    assert_eq!(state, expected, "MINA_PROTO_STATE_BODY state changed");
}

#[test]
fn test_mina_derive_token_id() {
    let state = MINA_DERIVE_TOKEN_ID.state();
    let expected = [
        fp_from_str("6192019453766080264591455948244350296532066491511280821771403784079613278630"),
        fp_from_str("3474280028978446563781013959252007045004226094384968366087940198662654278266"),
        fp_from_str(
            "20434002876694963787609307807174199928279086350854834006718281273564667456637",
        ),
    ];
    assert_eq!(state, expected, "MINA_DERIVE_TOKEN_ID state changed");
}

#[test]
fn test_mina_epoch_seed() {
    let state = MINA_EPOCH_SEED.state();
    let expected = [
        fp_from_str("7920024158807749362970659876749181530334941449960381128739613586571256360405"),
        fp_from_str(
            "13756862713999441076472977832321298402266591073703520273734381195492800342833",
        ),
        fp_from_str(
            "16931743843465107540110860558687538825985475311420101960428698400767332393906",
        ),
    ];
    assert_eq!(state, expected, "MINA_EPOCH_SEED state changed");
}

#[test]
fn test_mina_sideloaded_vk() {
    let state = MINA_SIDELOADED_VK.state();
    let expected = [
        fp_from_str(
            "27153629295534844750482612843518005572402188741101822965689207110291504095805",
        ),
        fp_from_str(
            "11073437601016088346212553894160581939150688827288603152461976873708720172824",
        ),
        fp_from_str("9169013693168830396847022454402673046094697740892173219744332585469764409612"),
    ];
    assert_eq!(state, expected, "MINA_SIDELOADED_VK state changed");
}

#[test]
fn test_mina_vrf_message() {
    let state = MINA_VRF_MESSAGE.state();
    let expected = [
        fp_from_str(
            "24101363367502572671624471609928959797353672294440762288404204895418767914646",
        ),
        fp_from_str("5171820881164007689309616183632792746219180909518238150637460314245246143263"),
        fp_from_str(
            "10979796915023089328772347959806029121878467684484216605075459818053899045444",
        ),
    ];
    assert_eq!(state, expected, "MINA_VRF_MESSAGE state changed");
}

#[test]
fn test_mina_vrf_output() {
    let state = MINA_VRF_OUTPUT.state();
    let expected = [
        fp_from_str("2251514781415689779315070305878469259850299612928948069881728941286436529416"),
        fp_from_str(
            "28445424317765931437563566658155841532256907311948842353165636913979445243675",
        ),
        fp_from_str("1697103740469522139030362533818365124680980524626250761960654638291888644330"),
    ];
    assert_eq!(state, expected, "MINA_VRF_OUTPUT state changed");
}

#[test]
fn test_coda_receipt_uc() {
    let state = CODA_RECEIPT_UC.state();
    let expected = [
        fp_from_str("2930292359494829300271368860633580634815819151887078160583250237349129726103"),
        fp_from_str(
            "15303314845540397914948764201521841781296890621466368017042313538410516382474",
        ),
        fp_from_str("8520568699315305732843613022173524514377597839978192694761879649747314556194"),
    ];
    assert_eq!(state, expected, "CODA_RECEIPT_UC state changed");
}

#[test]
fn test_coinbase_stack() {
    let state = COINBASE_STACK.state();
    let expected = [
        fp_from_str(
            "10365018507282248303752506973112854406071106890516858854157506926717812932750",
        ),
        fp_from_str(
            "19289691782405010481159082968251292806607879795611766141901748131065655579721",
        ),
        fp_from_str("8987039650233860747996941600635099179155585390854763935988086491644855810711"),
    ];
    assert_eq!(state, expected, "COINBASE_STACK state changed");
}

// Account Update Parameters
#[test]
fn test_mina_account_update_cons() {
    let state = MINA_ACCOUNT_UPDATE_CONS.state();
    let expected = [
        fp_from_str("7974184247425786365466969127827083941281743695327546149120833518746435921046"),
        fp_from_str("1079147682067570431747049877519099849334832444581201545961023544596733431550"),
        fp_from_str("9670106619202136718451303928765479503313491401619698334696903962327538130992"),
    ];
    assert_eq!(state, expected, "MINA_ACCOUNT_UPDATE_CONS state changed");
}

#[test]
fn test_mina_account_update_node() {
    let state = MINA_ACCOUNT_UPDATE_NODE.state();
    let expected = [
        fp_from_str(
            "15921812961830232432174711488904180713275251781093575291539345321597011303739",
        ),
        fp_from_str("5852213322332241594845871336918115662219071361771346507406094569679662937607"),
        fp_from_str(
            "21122827334147180286039671993443893600964526985496742826857975683524856341379",
        ),
    ];
    assert_eq!(state, expected, "MINA_ACCOUNT_UPDATE_NODE state changed");
}

#[test]
fn test_mina_account_update_stack_frame() {
    let state = MINA_ACCOUNT_UPDATE_STACK_FRAME.state();
    let expected = [
        fp_from_str("1223279431820750727612295994589444883292600761079562536688416996919972234987"),
        fp_from_str("1873141333924103856860857609363983758885824745969813373245393521390926426683"),
        fp_from_str("3550105212452130151915860825756512345408015936295894584118372238840612023788"),
    ];
    assert_eq!(
        state, expected,
        "MINA_ACCOUNT_UPDATE_STACK_FRAME state changed"
    );
}

#[test]
fn test_mina_account_update_stack_frame_cons() {
    let state = MINA_ACCOUNT_UPDATE_STACK_FRAME_CONS.state();
    let expected = [
        fp_from_str("2363089775097766730570162674460603870980415123701610894146069429352874281636"),
        fp_from_str("8717086429614898734892919627864489205116600585932141922995487227707208282057"),
        fp_from_str(
            "14660270392332597302006144557344641683528071714290878702086758222477469533211",
        ),
    ];
    assert_eq!(
        state, expected,
        "MINA_ACCOUNT_UPDATE_STACK_FRAME_CONS state changed"
    );
}

// ZkApp Parameters
#[test]
fn test_mina_zkapp_account() {
    let state = MINA_ZKAPP_ACCOUNT.state();
    let expected = [
        fp_from_str(
            "11742420651603425685690711434636216727968618158667382343736587130720645535016",
        ),
        fp_from_str(
            "20917169788479399921968659996772666237321879817943938162255353371266230737562",
        ),
        fp_from_str(
            "20221577186851444354528754069740362935513598751580381763045954351047955571417",
        ),
    ];
    assert_eq!(state, expected, "MINA_ZKAPP_ACCOUNT state changed");
}

#[test]
fn test_mina_zkapp_memo() {
    let state = MINA_ZKAPP_MEMO.state();
    let expected = [
        fp_from_str("2662735671148484138098041239517130399444285195614926917304994766121342901330"),
        fp_from_str("1889560324711062089177091328630260720221153765601231238715650562289804935970"),
        fp_from_str("4150523804923664151142435309968051550133270766858171566059780615187901817023"),
    ];
    assert_eq!(state, expected, "MINA_ZKAPP_MEMO state changed");
}

#[test]
fn test_mina_zkapp_uri() {
    let state = MINA_ZKAPP_URI.state();
    let expected = [
        fp_from_str("534822897390732927195976832726937157108052596941484097303405936433225931144"),
        fp_from_str(
            "21308674973525253012607500915181592359821899373849668837401701284134790635210",
        ),
        fp_from_str(
            "19235616568963430752220890547731083898076295596325584947617173371158207986317",
        ),
    ];
    assert_eq!(state, expected, "MINA_ZKAPP_URI state changed");
}

#[test]
fn test_mina_zkapp_event() {
    let state = MINA_ZKAPP_EVENT.state();
    let expected = [
        fp_from_str("4144672248660824652311280789227568759501644435839088465487215978090977152836"),
        fp_from_str(
            "16580012705864177241905923711864666027965216928284588602669501632136706453456",
        ),
        fp_from_str(
            "28268897103231723777184618409092967932555901943057586428182153116992131011025",
        ),
    ];
    assert_eq!(state, expected, "MINA_ZKAPP_EVENT state changed");
}

#[test]
fn test_mina_zkapp_events() {
    let state = MINA_ZKAPP_EVENTS.state();
    let expected = [
        fp_from_str(
            "22941690192200157010958144262626906691861453230235765939870625581651903942109",
        ),
        fp_from_str("8085194290973996063041942057794139208480036474122767282118588735695477304146"),
        fp_from_str(
            "26729904183313179836453835886592671283117737890095730465188585661277543615385",
        ),
    ];
    assert_eq!(state, expected, "MINA_ZKAPP_EVENTS state changed");
}

#[test]
fn test_mina_zkapp_seq_events() {
    let state = MINA_ZKAPP_SEQ_EVENTS.state();
    let expected = [
        fp_from_str(
            "20111532619758468729019280527752703188436440291616049387250019116440725105679",
        ),
        fp_from_str(
            "28272901079534355755544153990297346241256584111406088887261772513761686581936",
        ),
        fp_from_str(
            "11593971643819429831651280663135869674712971584194549509498204047075895747923",
        ),
    ];
    assert_eq!(state, expected, "MINA_ZKAPP_SEQ_EVENTS state changed");
}

// Network-specific Parameters
#[test]
fn test_coda_signature() {
    let state = CODA_SIGNATURE.state();
    let expected = [
        fp_from_str("6547874669265470003564181123405173756111990160585052594027544303901364349512"),
        fp_from_str(
            "22191763046611062479784309793717481299019591714391827084400612211604078633201",
        ),
        fp_from_str(
            "15360317550574394687602808211901764964514686767298144053612144955373862517277",
        ),
    ];
    assert_eq!(state, expected, "CODA_SIGNATURE state changed");
}

#[test]
fn test_testnet_zkapp_body() {
    let state = TESTNET_ZKAPP_BODY.state();
    let expected = [
        fp_from_str(
            "20037733640875789833090442509053816933966165101372309054048970230906793051053",
        ),
        fp_from_str("1106678471497583468621635190733109842219273971961053291385773425960251864224"),
        fp_from_str(
            "25565387364959491931899708566015584890804577695743228799735258954982776499278",
        ),
    ];
    assert_eq!(state, expected, "TESTNET_ZKAPP_BODY state changed");
}

#[test]
fn test_mina_signature_mainnet() {
    let state = MINA_SIGNATURE_MAINNET.state();
    let expected = [
        fp_from_str(
            "28597293842583882050529337819282358444728515448690248936274177901465134844489",
        ),
        fp_from_str(
            "13029865398778858891320837481651890827971447635226272051516204921834229015884",
        ),
        fp_from_str("2324960771278703080070347074343683653953770644553957353754880132143131569147"),
    ];
    assert_eq!(state, expected, "MINA_SIGNATURE_MAINNET state changed");
}

#[test]
fn test_mainnet_zkapp_body() {
    let state = MAINNET_ZKAPP_BODY.state();
    let expected = [
        fp_from_str(
            "10214915150831852734808709087755641273868350720962413399868532305813227181967",
        ),
        fp_from_str(
            "19231103515031626108540280352804904215178644233964839448405623573586547300771",
        ),
        fp_from_str("3202185325412846279878024015439663797323768206239602518916650099275135615824"),
    ];
    assert_eq!(state, expected, "MAINNET_ZKAPP_BODY state changed");
}

// Merkle Tree Parameters (0-35)
#[test]
fn test_mina_merkle_tree_0() {
    let state = MINA_MERKLE_TREE_0.state();
    let expected = [
        fp_from_str("8397268313679062041369959431253823194029931472150942928062160502284391094281"),
        fp_from_str(
            "24767884761786058961844271624848183563027832662151526765582126547150580343286",
        ),
        fp_from_str(
            "15520161476079946346223794435136450862321049619449569410496603974021593252201",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_0 state changed");
}

#[test]
fn test_mina_merkle_tree_1() {
    let state = MINA_MERKLE_TREE_1.state();
    let expected = [
        fp_from_str(
            "12373852158717286419843731546435335382149645091717657472272709119680142489615",
        ),
        fp_from_str(
            "13564003298811293044133692367818358732199958610489782205113648738971877309993",
        ),
        fp_from_str("5337043262085238844960907983211959910580364187637104432942748885155441259131"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_1 state changed");
}

#[test]
fn test_mina_merkle_tree_2() {
    let state = MINA_MERKLE_TREE_2.state();
    let expected = [
        fp_from_str(
            "15051812550454916172932351641588540140427950452718257831984749683884179659477",
        ),
        fp_from_str(
            "28383195182051628320454520194171815630993209993126957580698595309541504912011",
        ),
        fp_from_str("4277691878710291748308373204686233213493236676960343422888557635834505390473"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_2 state changed");
}

#[test]
fn test_mina_merkle_tree_3() {
    let state = MINA_MERKLE_TREE_3.state();
    let expected = [
        fp_from_str("6575607106027019342374634884807079936125440627705088279356425488661046931690"),
        fp_from_str("526224612349672274315011399400566806883023700724847451269254308717318755497"),
        fp_from_str("4003207773096098875040917033101823533304203798100315080652105415888406223352"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_3 state changed");
}

#[test]
fn test_mina_merkle_tree_4() {
    let state = MINA_MERKLE_TREE_4.state();
    let expected = [
        fp_from_str(
            "24963240007694741581504536598446662705874548366155724154174858737449434658477",
        ),
        fp_from_str("3025643334447992593201368502593388460692780911680818037147500927887943605498"),
        fp_from_str(
            "17577291971615136405466944877064852825800866932005309965300049909875838083076",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_4 state changed");
}

#[test]
fn test_mina_merkle_tree_5() {
    let state = MINA_MERKLE_TREE_5.state();
    let expected = [
        fp_from_str(
            "11625519336224216740433997623839523639549293720171430638848267458495647838261",
        ),
        fp_from_str(
            "14197827690168556134026805733901328807809311762374992007209622464122527394871",
        ),
        fp_from_str(
            "24909546339148248646747762490876591451430974658068769530058833648954096301456",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_5 state changed");
}

#[test]
fn test_mina_merkle_tree_6() {
    let state = MINA_MERKLE_TREE_6.state();
    let expected = [
        fp_from_str(
            "20496141241824212441237352225390586578798287226209999878764321364949616437960",
        ),
        fp_from_str("4155590369081069691345914612081918410248481482116023511739814856893535749559"),
        fp_from_str(
            "25280235590916436988517501437699802702512719636909687680088402215172012465734",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_6 state changed");
}

#[test]
fn test_mina_merkle_tree_7() {
    let state = MINA_MERKLE_TREE_7.state();
    let expected = [
        fp_from_str(
            "10298068926909347382132883731000773194312572157088286708479172422210086260995",
        ),
        fp_from_str(
            "14412862431845107093626156618901720148499279341044373322107997590840338638158",
        ),
        fp_from_str(
            "18738278293927842151520671915277777211638038066182255367951771829184874598427",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_7 state changed");
}

#[test]
fn test_mina_merkle_tree_8() {
    let state = MINA_MERKLE_TREE_8.state();
    let expected = [
        fp_from_str(
            "28632592040294076899303724277173923788865287530305670556694222869732793988004",
        ),
        fp_from_str(
            "14134336299867672225741933845142646509776280694779004162993533642733541282015",
        ),
        fp_from_str(
            "28037399410478206961594894531712592987717708818866863005767179190210518183828",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_8 state changed");
}

#[test]
fn test_mina_merkle_tree_9() {
    let state = MINA_MERKLE_TREE_9.state();
    let expected = [
        fp_from_str(
            "28322440793030270460307522165077799886504360112793805560745845785393893720792",
        ),
        fp_from_str("3680990636041985093510751436516424061735738733660682549379374982251492126646"),
        fp_from_str("7896824890513378496611723513283689788600212799489558200026291296328182622221"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_9 state changed");
}

#[test]
fn test_mina_merkle_tree_10() {
    let state = MINA_MERKLE_TREE_10.state();
    let expected = [
        fp_from_str("1478825754917601949043978332728751378179798684550333324122027096810422078645"),
        fp_from_str(
            "19955446483411426559697602372431961972639316232014088927090908136220581190127",
        ),
        fp_from_str(
            "22937261898125224845285209761802309482795401959887338893419704202246881755471",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_10 state changed");
}

#[test]
fn test_mina_merkle_tree_11() {
    let state = MINA_MERKLE_TREE_11.state();
    let expected = [
        fp_from_str("3025669655948979260146450778546273335663805909503078623788468939889184085065"),
        fp_from_str(
            "15993847354573651974906488175776892699808218007882158173582011967178852672755",
        ),
        fp_from_str("6728962834255630075044411175238909144357249010478126045082535701512206099100"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_11 state changed");
}

#[test]
fn test_mina_merkle_tree_12() {
    let state = MINA_MERKLE_TREE_12.state();
    let expected = [
        fp_from_str("4677165292950275428044379611682530196143565581952109747023715412205031133122"),
        fp_from_str(
            "21513899673761352699672092079955767010402365369617552725870531290648558067173",
        ),
        fp_from_str("5823386711670711136557441661686775439019939642112594994252954772574341048476"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_12 state changed");
}

#[test]
fn test_mina_merkle_tree_13() {
    let state = MINA_MERKLE_TREE_13.state();
    let expected = [
        fp_from_str("4137450619603133353679529278148472140169709465994051450094506977210968350741"),
        fp_from_str(
            "20777878603100506442428451439085789382190751853558867746947762486311334171694",
        ),
        fp_from_str(
            "27969424486066619381654224557897167292901506145568220790334189049235066613665",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_13 state changed");
}

#[test]
fn test_mina_merkle_tree_14() {
    let state = MINA_MERKLE_TREE_14.state();
    let expected = [
        fp_from_str("7462353831830439752760657933641455151117269082035084708085558232378403435178"),
        fp_from_str("1787244519320006617494344121814759180988090836648336932002915182844592150859"),
        fp_from_str("7682306724592829108592527309756770669512991144597346665572702844909814248134"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_14 state changed");
}

#[test]
fn test_mina_merkle_tree_15() {
    let state = MINA_MERKLE_TREE_15.state();
    let expected = [
        fp_from_str(
            "12849982892801603879697133836957604723592408109307896931233575279534184819695",
        ),
        fp_from_str(
            "27109175861581264256359157262042451197946512419680432940872313012019233881553",
        ),
        fp_from_str(
            "13815104575456051899693190094329831931582197916170277640933492195480185919492",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_15 state changed");
}

#[test]
fn test_mina_merkle_tree_16() {
    let state = MINA_MERKLE_TREE_16.state();
    let expected = [
        fp_from_str("6644594317394622409746632037064067639690803096240195936787541353591870145229"),
        fp_from_str(
            "11503303739151441813791807558499068063822922776334355558577285976158398952971",
        ),
        fp_from_str("9337125879737469121869180649342064063336965090821029559184624332617319461193"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_16 state changed");
}

#[test]
fn test_mina_merkle_tree_17() {
    let state = MINA_MERKLE_TREE_17.state();
    let expected = [
        fp_from_str(
            "16104414183099799176590675567463444044322697144434744941137050377134055108298",
        ),
        fp_from_str(
            "17358877713925634221311853575857896650017793698150943722268889332978652414223",
        ),
        fp_from_str("7234126597295300967301107936467282549695978865424211233580228640451399578381"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_17 state changed");
}

#[test]
fn test_mina_merkle_tree_18() {
    let state = MINA_MERKLE_TREE_18.state();
    let expected = [
        fp_from_str(
            "17030714373021103124485584722642520216144771996601835120521405613273080127695",
        ),
        fp_from_str(
            "26509950438240323122836106956137067463028271669140328718474012128935499432293",
        ),
        fp_from_str("9818889955393545543887790759741008577636326202190253983607321653665940190431"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_18 state changed");
}

#[test]
fn test_mina_merkle_tree_19() {
    let state = MINA_MERKLE_TREE_19.state();
    let expected = [
        fp_from_str("2270411531086562128123060093007384197084198453711890037231329803621300858719"),
        fp_from_str(
            "21192327485899043676835708468201514580084416399939158033379551466586666938111",
        ),
        fp_from_str(
            "16033989106273371309578845565218498940092011786025881685960320194668952032796",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_19 state changed");
}

#[test]
fn test_mina_merkle_tree_20() {
    let state = MINA_MERKLE_TREE_20.state();
    let expected = [
        fp_from_str(
            "24198821385641779512630219367024089801694762674171638444197433117564329069692",
        ),
        fp_from_str(
            "17399215024068249103454892742127252703846599060950907551709080066119343928674",
        ),
        fp_from_str("6797496550859701647209308902606013125966581359799331801777446461476502619719"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_20 state changed");
}

#[test]
fn test_mina_merkle_tree_21() {
    let state = MINA_MERKLE_TREE_21.state();
    let expected = [
        fp_from_str(
            "15260363122901687259348044007172490341059494245069079855983853109440904252201",
        ),
        fp_from_str(
            "21842382560395200222478365766257143008580907092198481707074219770380221343296",
        ),
        fp_from_str(
            "19876442709041612567866226719534012751674476537343454328432936802993088542055",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_21 state changed");
}

#[test]
fn test_mina_merkle_tree_22() {
    let state = MINA_MERKLE_TREE_22.state();
    let expected = [
        fp_from_str(
            "10163265845587027789352563667609980510844248093141311984644246724416434726269",
        ),
        fp_from_str(
            "13369107363202464111086659762824590159914641657154062683624651983805608000703",
        ),
        fp_from_str("259175261445126704640807316250901216510411826120741654939526054707099272571"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_22 state changed");
}

#[test]
fn test_mina_merkle_tree_23() {
    let state = MINA_MERKLE_TREE_23.state();
    let expected = [
        fp_from_str(
            "25461412540439968937539737336272713843660028687121793297700520674608023616092",
        ),
        fp_from_str(
            "10755893000209302712577283259218096030828962406298481790491258793159046533447",
        ),
        fp_from_str("8866387537961409494137849949417794325538964245944495058838310355591599158861"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_23 state changed");
}

#[test]
fn test_mina_merkle_tree_24() {
    let state = MINA_MERKLE_TREE_24.state();
    let expected = [
        fp_from_str(
            "10609060102237336747496673704618826236585593118726760088941627829244492026235",
        ),
        fp_from_str("6641853671926028367004819143507613339775735721213578606129119394262986889972"),
        fp_from_str("9176598236393962999771652301435919857376442200780273350376009302991599772639"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_24 state changed");
}

#[test]
fn test_mina_merkle_tree_25() {
    let state = MINA_MERKLE_TREE_25.state();
    let expected = [
        fp_from_str("8743110049886111146439413671080530377154252710287926017043338548443560750915"),
        fp_from_str(
            "18208706295395849104423491533560000448355322792628322573027110778397939938163",
        ),
        fp_from_str("2334327064299326913052261993744862803507079864714736582625500368508312407812"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_25 state changed");
}

#[test]
fn test_mina_merkle_tree_26() {
    let state = MINA_MERKLE_TREE_26.state();
    let expected = [
        fp_from_str("5402818389523730021623225031229797943489634186744070457165886896537635439065"),
        fp_from_str("3692115584159570188352953749250318597861823862334166671448537184963748474804"),
        fp_from_str("6395188055016804845192781987569287073677422173381065069566010090740896063910"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_26 state changed");
}

#[test]
fn test_mina_merkle_tree_27() {
    let state = MINA_MERKLE_TREE_27.state();
    let expected = [
        fp_from_str(
            "23003820738793392288354347717848227276238967632076473726115713990146403158695",
        ),
        fp_from_str("1543444712754301638361713310613005045560110448825211620606741790481631721785"),
        fp_from_str(
            "25595274127363608001744519321284450036093000747866790144244029941400855918414",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_27 state changed");
}

#[test]
fn test_mina_merkle_tree_28() {
    let state = MINA_MERKLE_TREE_28.state();
    let expected = [
        fp_from_str("1371490494959146551400088557556657100677286767912367568372761029147549710248"),
        fp_from_str("9732076291970119043155961140210107725581877672949581468560495192803834949972"),
        fp_from_str(
            "25880816593631896400945395886425311206250378130295778852027160152013445398428",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_28 state changed");
}

#[test]
fn test_mina_merkle_tree_29() {
    let state = MINA_MERKLE_TREE_29.state();
    let expected = [
        fp_from_str("5026966843162353429404633270081859806361860129654116093869475139616692501822"),
        fp_from_str(
            "14399077456078098196809897303466976088446801284524532382221120510698115179718",
        ),
        fp_from_str(
            "19173688776848337916142856590169111672948615917148745778746830767290062051975",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_29 state changed");
}

#[test]
fn test_mina_merkle_tree_30() {
    let state = MINA_MERKLE_TREE_30.state();
    let expected = [
        fp_from_str(
            "28464430208663194459267079800760461073547794864536029383346306150717295227411",
        ),
        fp_from_str(
            "28914474904259440678682001469840084119186056954595456498123242804122927256626",
        ),
        fp_from_str(
            "26203211860047183178968105249611310661035109372798510229151816437426414875870",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_30 state changed");
}

#[test]
fn test_mina_merkle_tree_31() {
    let state = MINA_MERKLE_TREE_31.state();
    let expected = [
        fp_from_str(
            "10406444365958122823322321566809921419619436370846100318015238317663537713508",
        ),
        fp_from_str(
            "19324027736939870254907794657369430751886546066724762856165204189536200502231",
        ),
        fp_from_str(
            "27910908481683556223061499853457824695327274417786658997051526597466420105059",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_31 state changed");
}

#[test]
fn test_mina_merkle_tree_32() {
    let state = MINA_MERKLE_TREE_32.state();
    let expected = [
        fp_from_str("1799233325885428173215288721205732918055309618518578057591098186182492814731"),
        fp_from_str("9573156486615047627167271384099786785626031045209045718144391096893253044237"),
        fp_from_str(
            "21011100500969260736212187791129169911216589801480194154796681238630801173303",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_32 state changed");
}

#[test]
fn test_mina_merkle_tree_33() {
    let state = MINA_MERKLE_TREE_33.state();
    let expected = [
        fp_from_str("3148460134537259154192780209825660438770489205065565102219398141630842726179"),
        fp_from_str(
            "10416076901773723654263170420888517757942744365709722512723508899712187445722",
        ),
        fp_from_str("9749070751601048371099954046702168187173097100747088073519924283619949053060"),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_33 state changed");
}

#[test]
fn test_mina_merkle_tree_34() {
    let state = MINA_MERKLE_TREE_34.state();
    let expected = [
        fp_from_str(
            "16036605154418397696690227738898261818934103448455753144542686857246049934251",
        ),
        fp_from_str(
            "13116195790811852398580983299275910910260911290232634459841017490947920635760",
        ),
        fp_from_str(
            "20914357145334136112903459144371894839046767011252485660349261543687439240515",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_34 state changed");
}

#[test]
fn test_mina_merkle_tree_35() {
    let state = MINA_MERKLE_TREE_35.state();
    let expected = [
        fp_from_str(
            "27121734445981217282939510097696253631089706813569006868445891968460368567974",
        ),
        fp_from_str("715381054565457856713917483767474125655250716827719934397684768568566660056"),
        fp_from_str(
            "22926360198644867909959646760416396768171909013888923485856060088656762220146",
        ),
    ];
    assert_eq!(state, expected, "MINA_MERKLE_TREE_35 state changed");
}

// Coinbase Merkle Tree Parameters (0-5)
#[test]
fn test_mina_cb_merkle_tree_0() {
    let state = MINA_CB_MERKLE_TREE_0.state();
    let expected = [
        fp_from_str("8045504582361301739415622676872039992523758242376047715582710838211313835037"),
        fp_from_str(
            "14119556581484270494862669385679132697170088480435154003600125395544356548682",
        ),
        fp_from_str("2012709992633119622410804207607862287926121316953166838330692822897259174594"),
    ];
    assert_eq!(state, expected, "MINA_CB_MERKLE_TREE_0 state changed");
}

#[test]
fn test_mina_cb_merkle_tree_1() {
    let state = MINA_CB_MERKLE_TREE_1.state();
    let expected = [
        fp_from_str(
            "12955548597001588636809395669457710944660125378325714791392293826809138181249",
        ),
        fp_from_str(
            "13577819559199691815864541467286532256289677624029161084821147822113243639553",
        ),
        fp_from_str(
            "18334253195322910156849939284536198782905970933702025691139682239895339083692",
        ),
    ];
    assert_eq!(state, expected, "MINA_CB_MERKLE_TREE_1 state changed");
}

#[test]
fn test_mina_cb_merkle_tree_2() {
    let state = MINA_CB_MERKLE_TREE_2.state();
    let expected = [
        fp_from_str("6863963043451948171173454560107741314829122680995507910413853990836863539452"),
        fp_from_str("9252960723748194366076890924614900676894810865806198667375086443359318340095"),
        fp_from_str(
            "12935152994917548290359159552054377349185197730945043183741211038353919694607",
        ),
    ];
    assert_eq!(state, expected, "MINA_CB_MERKLE_TREE_2 state changed");
}

#[test]
fn test_mina_cb_merkle_tree_3() {
    let state = MINA_CB_MERKLE_TREE_3.state();
    let expected = [
        fp_from_str(
            "28075543678705806143195600858979101333748064386532640070703602594045000091167",
        ),
        fp_from_str(
            "17346981294868700271774204102372995240391154091866862867323116003601379301976",
        ),
        fp_from_str(
            "22324716696364725326937135629784726085940011457286624450495311353688400678317",
        ),
    ];
    assert_eq!(state, expected, "MINA_CB_MERKLE_TREE_3 state changed");
}

#[test]
fn test_mina_cb_merkle_tree_4() {
    let state = MINA_CB_MERKLE_TREE_4.state();
    let expected = [
        fp_from_str(
            "15420803324236821962145606309611243483686488281721284996528075016907961985655",
        ),
        fp_from_str(
            "19062414841504572634880970229466689907153024865768755743273622243130306966397",
        ),
        fp_from_str(
            "10851358749017259882498933503411281550897340898344720357962545874919169887329",
        ),
    ];
    assert_eq!(state, expected, "MINA_CB_MERKLE_TREE_4 state changed");
}

#[test]
fn test_mina_cb_merkle_tree_5() {
    let state = MINA_CB_MERKLE_TREE_5.state();
    let expected = [
        fp_from_str(
            "24181632262934176716647969227712892934342561200415589528484735583227592895327",
        ),
        fp_from_str(
            "26020308252919930078525608832024417268592980943547939306418580167267040910667",
        ),
        fp_from_str("660114980140907798799127222409859879340540720201921032176884709692540869970"),
    ];
    assert_eq!(state, expected, "MINA_CB_MERKLE_TREE_5 state changed");
}
