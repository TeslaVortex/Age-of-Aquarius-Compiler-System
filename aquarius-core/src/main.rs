use aquarius_core::{
    canon::{print_seal, CANON_SHORT},
    clean_current::CleanCurrent,
    coherence::Coherence,
    collective_crown::{CollectiveCrown, CrownNode},
    inner_sun::InnerSun,
    mirror_lattice::{MirrorLattice, Node},
    mountain_root::MountainRoot,
    pour_engine::{PourEngine, PurePour},
    vortex::Field,
    zero_point::Mountain,
};

fn main() {
    // Canon seal first
    println!("{}", CANON_SHORT);
    println!("Mythotechnical Canon loaded.\n");

    println!("Present. Aware. Embodied.");
    println!("Follow-0.");
    println!("House primary. Cat present. Mountain steady.\n");

    println!("Initiating Pure Pour...\n");

    // 1. Mountain holds (Layer 0)
    let mountain = Mountain::hold();
    assert!(mountain.is_still());
    println!("✓ Mountain holds (Layer 0)");

    // 2. Clean Current activates (Layer 5)
    let mut current = CleanCurrent::new();
    current.initialize();
    current.activate(&mut Field::fresh());
    assert!(current.is_clean());
    println!("✓ Clean Current active (Layer 5)");

    // 3. Inner Sun heats (Layer 4)
    let mut sun = InnerSun::new();
    sun.initialize();
    sun.heat_through_presence(1.0);
    assert!(sun.is_sustainable());
    println!("✓ Inner Sun heating (Layer 4)");

    // 4. Mirror Lattice forms (Layer 3)
    let mut lattice = MirrorLattice::new();
    lattice.add_node_fresh(Node::new(1, "House"));
    lattice.add_node_fresh(Node::new(2, "Body"));
    lattice.add_node_fresh(Node::new(3, "Chain"));
    lattice.full_coherence();
    assert!(lattice.is_coherent());
    println!("✓ Mirror Lattice coherent (Layer 3)");

    // 5. Mountain Root embodies (Layer 7)
    let mut root = MountainRoot::new();
    root.embody();
    assert!(root.is_coherent());
    println!("✓ Mountain Root embodied (Layer 7)");

    // 6. Coherence collapses noise (Layer 2)
    let mut field = Field::noisy();
    Coherence::overwrite_noise(&mut field);
    assert!(field.complete && field.noise == 0.0);
    println!("✓ Noise collapsed, presence = 1.0 (Layer 2)");

    // 7. Collective Crown distributes (Layer 6)
    let mut crown = CollectiveCrown::new();
    crown.add_node(CrownNode::new(1, "House"));
    crown.add_node(CrownNode::new(2, "Body"));
    crown.add_node(CrownNode::new(3, "Chain"));
    crown.distribute_sovereignty();
    assert!(crown.is_sovereign());
    println!("✓ Collective Crown distributed (Layer 6)");

    // 8. Pure Pour completes (Layer 1)
    let mut field = Field::fresh();
    let engine = PourEngine;
    let result = engine.pour(&mut field).unwrap();

    println!("\nP = {}", result.p);
    println!("{}", result.message);
    println!("\n✓ Pure Pour complete (Layer 1)");
    println!("Mountain steady. Crown still.");
    println!("0-point holds.");
    println!("\n3-6-9 🪞🌞🔥👑🐱⛰️♾️");
    println!("The time is now.");

    // Canon seal at end
    print_seal();
}
