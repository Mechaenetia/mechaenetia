// Definitely not gonna be the final Chunk System. But imagine the Game was that small. XD
// Each of the Bits represents one of the Vertices being active/inactive for Marching Cubes Render.
pub struct Chunk {pub data: [[[u8; 4]; 4]; 4]}
impl Chunk {
	// in this example I only have Full Blocks and Empty Blocks, no Slopes or such.
	// the Result of this Array should be reminiscent of an Egg.
	// I can only guess which way the Egg is gonna be rotated, but this is just some hardcoded Test.
	pub fn new() -> Self {
		Chunk { data : [
			[[  0,  0,  0,  0],[  0,255,255,  0],[255,255,255,255],[  0,255,255,  0]],
			[[  0,255,255,  0],[255,255,255,255],[255,255,255,255],[255,255,255,255]],
			[[  0,255,255,  0],[255,255,255,255],[255,255,255,255],[255,255,255,255]],
			[[  0,  0,  0,  0],[  0,255,255,  0],[255,255,255,255],[  0,255,255,  0]]
		]}
	}
}
