use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Serialize)]
pub struct ProcedimentosPrincipais {
    co_procedimento: String,
    portaria: String,
}

pub fn ortopedia_procedimentos_principais(data: String) -> Vec<ProcedimentosPrincipais> {
    let mut output = vec![];

    for line in data.lines() {
        let strs = line.split(",").collect::<Vec<_>>();
        if strs[0] == "Procedimento:" {
            output.push(ProcedimentosPrincipais {
                co_procedimento: strs[1]
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .trim_end_matches('-')
                    .to_owned(),
                portaria: "1".to_owned(),
            });
        }
    }

    return output;
}

pub fn neurocirurgia_procedimentos_principais(data: String) -> Vec<ProcedimentosPrincipais> {
    let mut output = vec![];

    for line in data.lines() {
        let strs = line.split(",").collect::<Vec<_>>();
        if strs[0] == "Procedimento" {
            output.push(ProcedimentosPrincipais {
                co_procedimento: strs[1]
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .replace(".", "")
                    .replace("-", "")
                    .replace("\"", ""),
                portaria: "2".to_owned(),
            });
        }
    }

    return output;
}

pub fn oncologia_procedimentos_principais(data: String) -> Vec<ProcedimentosPrincipais> {
    let mut output = vec![];

    for line in data.lines().skip(1) {
        let co_procedimento = line
            .split(",")
            .next()
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap();

        output.push(ProcedimentosPrincipais {
            co_procedimento: co_procedimento.to_owned(),
            portaria: "3".to_owned(),
        });
    }

    return output;
}

pub fn ortopedia_procedimentos_sequenciais(data: String) -> HashSet<String> {
    let mut output = HashSet::new();
    for line in data.lines() {
        let cols: Vec<_> = line.split(',').collect();
        match cols[0] {
            "Procedimento:" => continue,
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
                + "1"
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
                + ",2\n",
        );
    }
    return output;
}

pub fn oncologia_procedimentos_sequenciais(data: String) -> HashSet<String> {
    let mut output = HashSet::new();
    for line in data.lines().skip(1) {
        let cols: Vec<_> = line.split(',').skip(1).collect();

        for sequencial in cols {
            output.insert(sequencial.replace("\"", "").replace(" ", "") + ",3\n");
        }
    }
    return output;
}

pub fn ortopedia_relations(data: String) -> String {
    let mut output = String::new();

    let mut co_procedimento = "";
    let mut cur_type = "";
    for line in data.lines() {
        let cols: Vec<_> = line.split(',').collect();
        match cols[0] {
            "Procedimento:" => {
                co_procedimento = cols[1]
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .trim_end_matches('-');
                continue;
            }
            "Sequencial:" => cur_type = "1",
            "Sequencial:(Compatíveis Seq.)" => cur_type = "2",
            _ => (),
        }
        let co_sequencial = cols[1]
            .split_whitespace()
            .next()
            .unwrap()
            .trim_end_matches('-');
        output = output + co_procedimento + "," + co_sequencial + "," + cur_type + ",1\n";
    }

    output.pop();
    return output;
}

pub fn neurocirurgia_relations(data: String) -> String {
    let mut output = String::new();

    let mut co_procedimento = String::new();

    for line in data.lines() {
        let cols: Vec<_> = line.split(',').collect();

        let codigo = cols[1]
            .split_whitespace()
            .next()
            .unwrap()
            .replace(".", "")
            .replace("-", "")
            .replace("\"", "");
        if cols[0] == "Procedimento" {
            co_procedimento = codigo;
            continue;
        }
        let co_sequencial = &codigo;
        output = output + co_procedimento.as_str() + "," + co_sequencial + ",1,2\n";
    }

    output.pop();
    return output;
}

pub fn oncologia_relations(data: String) -> String {
    let mut output = String::new();

    for line in data.lines().skip(1) {
        let cols: Vec<_> = line.split(',').collect();
        let co_procedimento = cols[0].split_whitespace().next().unwrap();

        for co_sequencial in cols.into_iter().skip(1) {
            output = output
                + co_procedimento
                + ","
                + co_sequencial.replace("\"", "").replace(" ", "").as_str()
                + ",1,3\n";
        }
    }

    output.pop();
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

        let output = vec![
            ProcedimentosPrincipais {
                co_procedimento: "0408010029".to_owned(),
                portaria: "1".to_owned(),
            },
            ProcedimentosPrincipais {
                co_procedimento: "0408010045".to_owned(),
                portaria: "1".to_owned(),
            },
        ];
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
            "0401020010,1\n".to_string(),
            "0401020029,1\n".to_string(),
            "0401020037,1\n".to_string(),
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
            "0408030372,2\n".to_string(),
            "0403010098,2\n".to_string(),
            "0403010101,2\n".to_string(),
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
            "0409050091,3\n".to_string(),
            "0409020168,3\n".to_string(),
            "0416020020,3\n".to_string(),
            "0416020232,3\n".to_string(),
            "0416020259,3\n".to_string(),
            "0416010040,3\n".to_string(),
            "0416020020,3\n".to_string(),
            "416020224,3\n".to_string(),
        ]);

        assert_eq!(
            oncologia_procedimentos_sequenciais(input.to_string()),
            output
        );
    }

    #[test]
    fn test7() {
        let input = "Procedimento:,0408010029 - ARTRODESE DE GRANDES ARTICULAÇÕES ESCAPULO-UMERAIS
Sequencial:,0408060387 - RETIRADA DE PROTESE DE SUBSTITUICAO DE GRANDES ARTICULACOES (OMBRO / COTOVELO / QUADRIL / JOELHO)
,0408060344 - RETIRADA DE ESPACADORES / OUTROS MATERIAIS
,0408060352 - RETIRADA DE FIO OU PINO INTRA-OSSEO";

        let output =
            "0408010029,0408060387,1,1\n0408010029,0408060344,1,1\n0408010029,0408060352,1,1";

        assert_eq!(ortopedia_relations(input.to_string()), output);
    }

    #[test]
    fn test8() {
        let input = "Procedimento:,0408020016 - AMPUTACAO / DESARTICULACAO DE MAO E PUNHO
Sequencial:,0408060514 - TRANSPLANTE MUSCULO-CUTANEO C/ MICRO-ANASTOMOSE NO TRONCO / EXTREMIDADE
Sequencial:(Compatíveis Seq.),0401020010 - ENXERTO COMPOSTO
,0401020029 - ENXERTO DERMO-EPIDÉRMICO";

        let output =
            "0408020016,0408060514,1,1\n0408020016,0401020010,2,1\n0408020016,0401020029,2,1";

        assert_eq!(ortopedia_relations(input.to_string()), output);
    }

    #[test]
    fn test9() {
        let input = "Procedimento,04.03.01.003-9 - Craniotomia descompressiva da fossa posterior
Sequencial,04.08.03.037-2 - Descompressão óssea na junção crânio-cervical via posterior c/ ampliação dural
,04.03.01.009-8 - Derivação ventricular externar-subgaleal externa
,04.03.01.010-1 - Derivação ventricular para peritôneo / átrio / pleura / raque";

        let output =
            "0403010039,0408030372,1,2\n0403010039,0403010098,1,2\n0403010039,0403010101,1,2";

        assert_eq!(neurocirurgia_relations(input.to_string()), output);
    }
    #[test]
    fn test10() {
        let input = r#"Procedimento,Sequenciais
0416010016 - Amputação de pênis em oncologia,"0409050091, 0409020168, 0416020020, 0416020232, 0416020259"
0416010024 - Cistectomia total e derivação em 1 só tempo em oncologia,"0416010040, 0416020020""#;

        let output = "0416010016,0409050091,1,3\n0416010016,0409020168,1,3\n0416010016,0416020020,1,3\n0416010016,0416020232,1,3\n0416010016,0416020259,1,3\n0416010024,0416010040,1,3\n0416010024,0416020020,1,3";

        assert_eq!(oncologia_relations(input.to_string()), output);
    }
}
