// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn print_tile(tile: Tile) {
    use Tile::*;
    match tile {
        Brick(brick @ BrickStyle::Red | brick @ BrickStyle::Gray) => {
            println!("The brick color is {:?}", brick)
        }
        Brick(style) => println!("{:?} brick", style),
        Water(p) if p.0 < 10 => println!("Water pressure level: {:?}", p.0),
        Water(_) => println!("High water pressure!"),
        Dirt | Grass | Sand => println!("Ground tile"),
        Treasure(TreasureChest {
            content: TreasureItem::Gold,
            amount: a,
        }) if a >= 100 => println!("Lots of gold!"),
        _ => (),
    }
}

fn main() {
    let tile = Tile::Water(Pressure(30));
    print_tile(tile);
    let tile = Tile::Brick(BrickStyle::Gray);
    print_tile(tile);
    let tile = Tile::Brick(BrickStyle::Dungeon);
    print_tile(tile);
    let tile = Tile::Treasure(TreasureChest {
        amount: 110,
        content: TreasureItem::Gold,
    });
    print_tile(tile);
}
