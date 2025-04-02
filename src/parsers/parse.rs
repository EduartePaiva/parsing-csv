use std::collections::HashSet;

pub fn ortopedia_procedimentos_principais(data: String) -> String {
    let mut output = String::new();

    for line in data.lines() {
        let strs = line.split(",").collect::<Vec<_>>();
        if strs[0] == "Procedimento:" {
            output = output
                + strs[1]
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .trim_end_matches('-')
                + ",1\n"
        }
    }
    output.pop();

    return output;
}

pub fn neurocirurgia_procedimentos_principais(data: String) -> String {
    let mut output = String::new();

    for line in data.lines() {
        let strs = line.split(",").collect::<Vec<_>>();
        if strs[0] == "Procedimento" {
            output = output
                + strs[1]
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .replace(".", "")
                    .replace("-", "")
                    .replace("\"", "")
                    .as_str()
                + ",2\n"
        }
    }
    output.pop();

    return output;
}

pub fn oncologia_procedimentos_principais(data: String) -> String {
    let mut output = String::new();

    for line in data.lines().skip(1) {
        let co_procedimento = line
            .split(",")
            .next()
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap();

        output = output + co_procedimento + ",3\n"
    }
    output.pop();

    return output;
}

pub fn ortopedia_procedimentos_sequenciais(data: String) -> HashSet<String> {
    let mut output = HashSet::new();
    let mut cur_type = "";
    for line in data.lines() {
        let cols: Vec<_> = line.split(',').collect();
        match cols[0] {
            "Procedimento:" => continue,
            "Sequencial:" => cur_type = "1",
            "Sequencial:(Compatíveis Seq.)" => cur_type = "2",
            _ => (),
        }
        output.insert(
            cols[1]
                .split_whitespace()
                .next()
                .unwrap()
                .trim_end_matches('-')
                .to_string()
                + ","
                + cur_type
                + "\n",
        );
    }
    return output;
}

pub fn neurocirurgia_procedimentos_sequenciais(data: String) -> HashSet<String> {
    let mut output = HashSet::new();
    for line in data.lines() {
        let cols: Vec<_> = line.split(',').collect();
        if cols[0] == "Procedimento" {
            continue;
        }
        output.insert(
            cols[1]
                .split_whitespace()
                .next()
                .unwrap()
                .replace(".", "")
                .replace("-", "")
                .replace("\"", "")
                + ",1\n",
        );
    }
    return output;
}

pub fn oncologia_procedimentos_sequenciais(data: String) -> HashSet<String> {
    let mut output = HashSet::new();
    for line in data.lines().skip(1) {
        let cols: Vec<_> = line.split(',').skip(1).collect();

        for sequencial in cols {
            output.insert(sequencial.replace("\"", "").replace(" ", "") + ",1\n");
        }
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = "Procedimento:,0408010029 - ARTRODESE DE GRANDES ARTICULAÇÕES ESCAPULO-UMERAIS
Sequencial:,0408060387 - RETIRADA DE PROTESE DE SUBSTITUICAO DE GRANDES ARTICULACOES (OMBRO / COTOVELO / QUADRIL / JOELHO)
,0408060344 - RETIRADA DE ESPACADORES / OUTROS MATERIAIS
,0408060352 - RETIRADA DE FIO OU PINO INTRA-OSSEO
,0408060379 - RETIRADA DE PLACA E/OU PARAFUSOS
Procedimento:,0408010045 - ARTROPLASTIA ESCAPULO-UMERAL PARCIAL
Sequencial:,0408060344 - RETIRADA DE ESPACADORES / OUTROS MATERIAIS
,0408060352 - RETIRADA DE FIO OU PINO INTRA-OSSEO
,0408060379 - RETIRADA DE PLACA E/OU PARAFUSOS";

        let output = "0408010029,1
0408010045,1";
        assert_eq!(
            ortopedia_procedimentos_principais(input.to_string()),
            output
        )
    }

    #[test]
    fn test2() {
        let input = "Procedimento,04.03.01.003-9 - Craniotomia descompressiva da fossa posterior
Sequencial,04.08.03.037-2 - Descompressão óssea na junção crânio-cervical via posterior c/ ampliação dural
,04.03.01.009-8 - Derivação ventricular externar-subgaleal externa
,04.03.01.010-1 - Derivação ventricular para peritôneo / átrio / pleura / raque
,04.03.01.034-9 - Trepanação craniana para propedêutica neurocirúrgica / implante para monitorização pic
Procedimento,04.03.01.002-0 - Craniotomia descompressiva
Sequencial,04.03.01.009-8 - Derivação ventricular externar-subgaleal externa
,04.03.01.010-1 - Derivação ventricular para peritôneo / átrio / pleura / raque
,04.03.01.034-9 - Trepanação craniana para propedêutica neurocirúrgica/ implante para monitorização pic";

        let output = "0403010039,2\n0403010020,2";

        assert_eq!(
            neurocirurgia_procedimentos_principais(input.to_string()),
            output
        );
    }

    #[test]
    fn test3() {
        let input = r#"Procedimento,Sequenciais
0416010016 - Amputação de pênis em oncologia,"0409050091, 0409020168, 0416020020, 0416020232, 0416020259"
0416010024 - Cistectomia total e derivação em 1 só tempo em oncologia,"0416010040, 0416020020"
0416010075 - Nefrectomia total em oncologia,416020224"#;

        let ourput = "0416010016,3\n0416010024,3\n0416010075,3";

        assert_eq!(
            oncologia_procedimentos_principais(input.to_string()),
            ourput
        );
    }

    #[test]
    fn test4() {
        let input = "Procedimento:,0408010061 - ARTROPLASTIA ESCAPULO-UMERAL TOTAL - REVISÃO/RECONSTRUÇÃO
Sequencial:,0408060387 - RETIRADA DE PROTESE DE SUBSTITUICAO DE GRANDES ARTICULACOES (OMBRO / COTOVELO / QUADRIL / JOELHO)
,0408060344 - RETIRADA DE ESPACADORES / OUTROS MATERIAIS
,0408060352 - RETIRADA DE FIO OU PINO INTRA-OSSEO
,0408060620 - TRATAMENTO CIRURGICO DE INFECCAO POS-ARTROPLASTIA (GRANDES ARTICULACOES)
Procedimento:,0408010070 - DESARTICULACAO DA ARTICULACAO ESCAPULO-UMERAL
Sequencial:,0408060514 - TRANSPLANTE MUSCULO-CUTANEO C/ MICRO-ANASTOMOSE NO TRONCO / EXTREMIDADE
Sequencial:(Compatíveis Seq.),0401020010 - ENXERTO COMPOSTO
,0401020029 - ENXERTO DERMO-EPIDÉRMICO
,0401020037 - ENXERTO LIVRE DE PELE TOTAL";

        let output: HashSet<String> = HashSet::from_iter(vec![
            "0401020010,2\n".to_string(),
            "0401020029,2\n".to_string(),
            "0401020037,2\n".to_string(),
            "0408060344,1\n".to_string(),
            "0408060352,1\n".to_string(),
            "0408060387,1\n".to_string(),
            "0408060514,1\n".to_string(),
            "0408060620,1\n".to_string(),
        ]);

        assert_eq!(
            ortopedia_procedimentos_sequenciais(input.to_string()),
            output
        );
    }

    #[test]
    fn test5() {
        let input = "Procedimento,04.03.01.003-9 - Craniotomia descompressiva da fossa posterior
Sequencial,04.08.03.037-2 - Descompressão óssea na junção crânio-cervical via posterior c/ ampliação dural
,04.03.01.009-8 - Derivação ventricular externar-subgaleal externa
,04.03.01.010-1 - Derivação ventricular para peritôneo / átrio / pleura / raque";

        let output: HashSet<String> = HashSet::from_iter(vec![
            "0408030372,1\n".to_string(),
            "0403010098,1\n".to_string(),
            "0403010101,1\n".to_string(),
        ]);

        assert_eq!(
            neurocirurgia_procedimentos_sequenciais(input.to_string()),
            output
        );
    }

    #[test]
    fn test6() {
        let input = r#"Procedimento,Sequenciais
0416010016 - Amputação de pênis em oncologia,"0409050091, 0409020168, 0416020020, 0416020232, 0416020259"
0416010024 - Cistectomia total e derivação em 1 só tempo em oncologia,"0416010040, 0416020020"
0416010075 - Nefrectomia total em oncologia,416020224"#;

        let output: HashSet<String> = HashSet::from_iter(vec![
            "0409050091,1\n".to_string(),
            "0409020168,1\n".to_string(),
            "0416020020,1\n".to_string(),
            "0416020232,1\n".to_string(),
            "0416020259,1\n".to_string(),
            "0416010040,1\n".to_string(),
            "0416020020,1\n".to_string(),
            "416020224,1\n".to_string(),
        ]);

        assert_eq!(
            oncologia_procedimentos_sequenciais(input.to_string()),
            output
        );
    }
}
