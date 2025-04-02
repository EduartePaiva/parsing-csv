use parsing_pcrocedimentos::parsers::{
    file_manipulation::{read_the_file, save_the_file},
    parse::{
        neurocirurgia_procedimentos_principais, neurocirurgia_procedimentos_sequenciais,
        oncologia_procedimentos_principais, oncologia_procedimentos_sequenciais,
        ortopedia_procedimentos_principais, ortopedia_procedimentos_sequenciais,
    },
};

fn main() {
    // Trabalhando na primeira tabela
    let ortopedia = read_the_file("./src/ortopedia.csv");
    let neurocirurgia = read_the_file("./src/neurocirurgia.csv");
    let oncologia = read_the_file("./src/oncologia.csv");

    let procedimentos_principais = ortopedia_procedimentos_principais(ortopedia.clone())
        + "\n"
        + neurocirurgia_procedimentos_principais(neurocirurgia.clone()).as_str()
        + "\n"
        + oncologia_procedimentos_principais(oncologia.clone()).as_str();

    save_the_file(
        "./src/procedimentos_principais.csv",
        procedimentos_principais,
    );

    // Trabalhando tabela procedimentos sequenciais
    let mut procedimentos_sequenciais = ortopedia_procedimentos_sequenciais(ortopedia.clone());
    procedimentos_sequenciais.extend(neurocirurgia_procedimentos_sequenciais(neurocirurgia));
    procedimentos_sequenciais.extend(oncologia_procedimentos_sequenciais(oncologia.clone()));

    let mut procedimentos_sequenciais: String = procedimentos_sequenciais.into_iter().collect();
    procedimentos_sequenciais.pop();
    save_the_file(
        "./src/procedimentos_sequenciais.csv",
        procedimentos_sequenciais,
    );
}
