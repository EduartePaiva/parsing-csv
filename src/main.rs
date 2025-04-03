use parsing_pcrocedimentos::parsers::{
    file_manipulation::{read_the_file, save_the_file},
    parse::{
        neurocirurgia_procedimentos_principais, neurocirurgia_procedimentos_sequenciais,
        neurocirurgia_relations, oncologia_procedimentos_principais,
        oncologia_procedimentos_sequenciais, oncologia_relations,
        ortopedia_procedimentos_principais, ortopedia_procedimentos_sequenciais,
        ortopedia_relations,
    },
};

fn main() {
    // Trabalhando na primeira tabela
    let ortopedia = read_the_file("./src/ortopedia.csv");
    let neurocirurgia = read_the_file("./src/neurocirurgia.csv");
    let oncologia = read_the_file("./src/oncologia.csv");

    let mut procedimentos_principais = ortopedia_procedimentos_principais(ortopedia.clone());
    procedimentos_principais.extend(neurocirurgia_procedimentos_principais(
        neurocirurgia.clone(),
    ));
    procedimentos_principais.extend(odontologia_procedimentos_principais(neurocirurgia.clone()));

    let procedimentos_principais = serde_json::to_string_pretty(&procedimentos_principais).unwrap();

    save_the_file(
        "./src/procedimentos_principais.csv",
        procedimentos_principais,
    );

    // Trabalhando tabela procedimentos sequenciais
    let mut procedimentos_sequenciais = ortopedia_procedimentos_sequenciais(ortopedia.clone());
    procedimentos_sequenciais.extend(neurocirurgia_procedimentos_sequenciais(
        neurocirurgia.clone(),
    ));
    procedimentos_sequenciais.extend(oncologia_procedimentos_sequenciais(oncologia.clone()));

    let mut procedimentos_sequenciais: String = procedimentos_sequenciais.into_iter().collect();
    procedimentos_sequenciais.pop();
    save_the_file(
        "./src/procedimentos_sequenciais.csv",
        procedimentos_sequenciais,
    );

    // Trabalhando tabela relações
    let relacoes = ortopedia_relations(ortopedia)
        + "\n"
        + neurocirurgia_relations(neurocirurgia).as_str()
        + "\n"
        + oncologia_relations(oncologia).as_str();

    save_the_file("./src/relacoes.csv", relacoes);
}
