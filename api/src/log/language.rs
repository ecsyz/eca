// https://github.com/ArtificialQualia/PyEveLiveDPS/blob/master/PyEveLiveDPS/logreader.py
// enum LogLanguage {
//     Russian,
//     English,
//     French,
//     German,
//     Japanese,
//     Chinese,
// }

lazy_static! {
    static ref LOGLANGUAGE: HashMap<&'static str, LogLanguage> = generate_language();
}

#[derive(Debug)]
pub struct LogLanguage {
    language:            &'static str,
    character:           &'static str,
    session_time:        &'static str,
    damage_out:          &'static str,
    damage_in:           &'static str,
    armor_repaired_out:  &'static str,
    hull_repaired_out:   &'static str,
    shield_boosted_out:  &'static str,
    armor_repaired_in:   &'static str,
    hull_repaired_in:    &'static str,
    shield_boosted_in:   &'static str,
    cap_transfered_out:  &'static str,
    cap_neutralized_out: &'static str,
    nos_recieved:        &'static str,
    cap_transfered_in:   &'static str,
    cap_neutralized_in:  &'static str,
    nos_taken:           &'static str,
    mined:               &'static str,
}

impl LogLanguage {
    pub fn new(
        language: &'static str,
        character: &'static str,
        session_time: &'static str,
        // pilot_and_weapon: &'static str,
        damage_out: &'static str,
        damage_in: &'static str,
        armor_repaired_out: &'static str,
        hull_repaired_out: &'static str,
        shield_boosted_out: &'static str,
        armor_repaired_in: &'static str,
        hull_repaired_in: &'static str,
        shield_boosted_in: &'static str,
        cap_transfered_out: &'static str,
        cap_neutralized_out: &'static str,
        nos_recieved: &'static str,
        cap_transfered_in: &'static str,
        cap_neutralized_in: &'static str,
        nos_taken: &'static str,
        mined: &'static str,
    ) -> Self {
        Self {
            language,
            character,
            session_time,
            // pilot_and_weapon,
            damage_out,
            damage_in,
            armor_repaired_out,
            hull_repaired_out,
            shield_boosted_out,
            armor_repaired_in,
            hull_repaired_in,
            shield_boosted_in,
            cap_transfered_out,
            cap_neutralized_out,
            nos_recieved,
            cap_transfered_in,
            cap_neutralized_in,
            nos_taken,
            mined,
        }
    }
}

pub fn generate_language() -> HashMap<&'static str, LogLanguage> {
    let mut list: HashMap<&'static str, LogLanguage> = HashMap::with_capacity(6);
    list.insert(
        "english",
        LogLanguage::new(
            "english",
            r#"  Listener: "#,
            r#"  Session Started: "#,
            r#"to"#,
            r#"from"#,
            r#"remote armor repaired to"#,
            r#"remote hull repaired to"#,
            r#"remote shield boosted to"#,
            r#"remote armor repaired by"#,
            r#"remote hull repaired by"#,
            r#"remote shield boosted by"#,
            r#"remote capacitor transmitted to"#,
            r#"energy neutralized"#,
            r#"energy drained from"#,
            r#"remote capacitor transmitted by"#,
            r#"energy neutralized"#,
            r#"energy drained to"#,
            r#" (mining)"#,
        ),
    );

    list.insert(
        "russian",
        LogLanguage::new(
            "russian",
            r#"  Слушатель: "#,
            r#"  Сеанс начат: "#,
            r#"на"#,
            r#"из"#,
            r#"единиц запаса прочности брони отремонтировано"#,
            r#"единиц запаса прочности корпуса отремонтировано"#,
            r#"единиц запаса прочности щитов накачано"#,
            r#"единиц запаса прочности брони получено дистанционным ремонтом от"#,
            r#"единиц запаса прочности корпуса получено дистанционным ремонтом от"#,
            r#"единиц запаса прочности щитов получено накачкой от"#,
            r#"единиц запаса энергии накопителя отправлено в"#,
            r#"энергии нейтрализовано"#,
            r#"энергии извлечено из"#,
            r#"единиц запаса энергии накопителя получено от"#,
            r#"энергии нейтрализовано"#,
            r#"энергии извлечено и передано"#,
            r#" (mining)"#,
        ),
    );
    list.insert(
        "french",
        LogLanguage::new(
            "french",
            r#"  Auditeur: "#,
            r#"  Session commencée: "#,
            r#"à"#,
            r#"de"#,
            r#"points de blindage transférés à distance à"#,
            r#"points de structure transférés à distance à"#,
            r#"points de boucliers transférés à distance à"#,
            r#"points de blindage réparés à distance par"#,
            r#"points de structure réparés à distance par"#,
            r#"points de boucliers transférés à distance par"#,
            r#"points de capaciteur transférés à distance à"#,
            r#"d'énergie neutralisée en faveur de"#,
            r#"d'énergie siphonnée aux dépens de"#,
            r#"points de capaciteur transférés à distance par"#,
            r#"d'énergie neutralisée aux dépens de"#,
            r#"d'énergie siphonnée en faveur de"#,
            r#" (mining)"#,
        ),
    );
    list.insert(
        "german",
        LogLanguage::new(
            "german",
            r#"  Empfänger: "#,
            r#"  Sitzung gestartet: "#,
            r#"nach"#,
            r#"von"#,
            r#"Panzerungs-Fernreparatur zu"#,
            r#"Rumpf-Fernreparatur zu"#,
            r#"Schildfernbooster aktiviert zu"#,
            r#"Panzerungs-Fernreparatur von"#,
            r#"Rumpf-Fernreparatur von"#,
            r#"Schildfernbooster aktiviert von"#,
            r#"Fernenergiespeicher übertragen zu"#,
            r#"Energie neutralisiert"#,
            r#"Energie transferiert von"#,
            r#"Fernenergiespeicher übertragen von"#,
            r#"Energie neutralisiert"#,
            r#"Energie transferiert zu"#,
            r#" (mining)"#,
        ),
    );
    list.insert(
        "japanese",
        LogLanguage::new(
            "japanese",
            r#"  傍聴者: "#,
            r#"  セッション開始: "#,
            r#"対象:"#,
            r#"攻撃者:"#,
            r#"remote armor repaired to"#,
            r#"remote hull repaired to"#,
            r#"remote shield boosted to"#,
            r#"remote armor repaired by"#,
            r#"remote hull repaired by"#,
            r#"remote shield boosted by"#,
            r#"remote capacitor transmitted to"#,
            r#"エネルギーニュートラライズ 対象"#,
            r#"エネルギードレイン 対象"#,
            r#"remote capacitor transmitted by"#,
            r#"のエネルギーが解放されました<"#,
            r#"エネルギードレイン 攻撃者"#,
            r#" (mining)"#,
        ),
    );
    list.insert(
        "chinese",
        LogLanguage::new(
            "chinese",
            r#"  收听者: "#,
            r#"  进程开始: "#,
            r#"对"#,
            r#"来自"#,
            r#"远程装甲维修量至"#,
            r#"远程结构维修量至"#,
            r#"远程护盾回充增量至"#,
            r#"远程装甲维修量由"#,
            r#"远程结构维修量由"#,
            r#"远程护盾回充增量由"#,
            r#"远程电容传输至"#,
            r#"能量中和"#,
            r#"被从"#,
            r#"远程电容传输量由"#,
            r#"能量中和"#,
            r#"被吸取到"#,
            r#" (mining)"#,
        ),
    );

    list
}
