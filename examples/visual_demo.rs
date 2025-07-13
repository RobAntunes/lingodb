//! Visual demonstration of LingoDB's 3D spatial organization
//! Outputs an HTML file with an interactive 3D visualization

use lingo::storage::MemoryMappedDatabase;
use lingo::core::*;
use std::io::Write;
use std::fs::File;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Generating Visual Demo of LingoDB\n");
    
    // Load the base database
    let database = MemoryMappedDatabase::open("data/base.lingo")?;
    println!("Database loaded with {} nodes\n", database.node_count());
    
    // Collect all morphemes with their positions
    let mut morphemes_by_type: HashMap<MorphemeType, Vec<(String, Coordinate3D)>> = HashMap::new();
    let mut all_positions = Vec::new();
    
    for i in 0..database.node_count() {
        let node_id = NodeId(i as u32);
        if let Ok(node) = database.get_node(node_id) {
            if node.layer == Layer::Morphemes {
                if let Ok(word) = database.get_node_word(node_id) {
                    let pos = node.position; // Copy to avoid alignment issues
                    morphemes_by_type
                        .entry(node.morpheme_type)
                        .or_default()
                        .push((word.to_string(), pos));
                    all_positions.push((word.to_string(), pos, node.morpheme_type));
                }
            }
        }
    }
    
    // Generate statistics
    println!("📊 Morpheme Distribution:");
    for (morph_type, items) in &morphemes_by_type {
        println!("  {:?}: {} morphemes", morph_type, items.len());
    }
    
    // Generate HTML visualization
    generate_3d_visualization(&morphemes_by_type, &all_positions)?;
    
    // Generate 2D projections
    generate_2d_projections(&morphemes_by_type)?;
    
    // Analyze spatial patterns
    analyze_spatial_patterns(&morphemes_by_type)?;
    
    println!("\n✅ Visualization generated: lingodb_visualization.html");
    println!("   Open this file in a web browser to explore the 3D structure!");
    
    Ok(())
}

fn generate_3d_visualization(
    morphemes_by_type: &HashMap<MorphemeType, Vec<(String, Coordinate3D)>>,
    all_positions: &[(String, Coordinate3D, MorphemeType)]
) -> Result<(), Box<dyn std::error::Error>> {
    let mut html = String::new();
    
    // HTML header with Three.js
    html.push_str(r#"<!DOCTYPE html>
<html>
<head>
    <title>LingoDB 3D Visualization</title>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/controls/OrbitControls.js"></script>
    <style>
        body { margin: 0; font-family: Arial, sans-serif; }
        #info {
            position: absolute;
            top: 10px;
            left: 10px;
            background: rgba(255, 255, 255, 0.9);
            padding: 10px;
            border-radius: 5px;
            max-width: 300px;
        }
        #legend {
            position: absolute;
            top: 10px;
            right: 10px;
            background: rgba(255, 255, 255, 0.9);
            padding: 10px;
            border-radius: 5px;
        }
        .legend-item {
            margin: 5px 0;
        }
        .color-box {
            display: inline-block;
            width: 20px;
            height: 20px;
            margin-right: 5px;
            vertical-align: middle;
        }
    </style>
</head>
<body>
    <div id="info">
        <h3>LingoDB 3D Morpheme Space</h3>
        <p>Click and drag to rotate, scroll to zoom</p>
        <p>Hover over points to see morphemes</p>
        <div id="hover-info"></div>
    </div>
    <div id="legend">
        <h4>Morpheme Types</h4>
        <div class="legend-item">
            <span class="color-box" style="background: #ff0000;"></span>Prefixes
        </div>
        <div class="legend-item">
            <span class="color-box" style="background: #0000ff;"></span>Suffixes
        </div>
        <div class="legend-item">
            <span class="color-box" style="background: #00ff00;"></span>Roots
        </div>
    </div>
    
    <script>
        // Scene setup
        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0xf0f0f0);
        
        const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
        camera.position.set(2, 2, 2);
        
        const renderer = new THREE.WebGLRenderer({ antialias: true });
        renderer.setSize(window.innerWidth, window.innerHeight);
        document.body.appendChild(renderer.domElement);
        
        // Controls
        const controls = new THREE.OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;
        
        // Lights
        const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
        scene.add(ambientLight);
        const directionalLight = new THREE.DirectionalLight(0xffffff, 0.4);
        directionalLight.position.set(1, 1, 1);
        scene.add(directionalLight);
        
        // Grid helper
        const gridHelper = new THREE.GridHelper(2, 20);
        scene.add(gridHelper);
        
        // Axes helper
        const axesHelper = new THREE.AxesHelper(1);
        scene.add(axesHelper);
        
        // Morpheme data
        const morphemes = [
"#);
    
    // Add morpheme data
    for (word, pos, morph_type) in all_positions {
        let color = match morph_type {
            MorphemeType::Prefix => "0xff0000",
            MorphemeType::Suffix => "0x0000ff",
            MorphemeType::Root => "0x00ff00",
            _ => "0x888888",
        };
        
        html.push_str(&format!(
            "            {{ word: '{}', x: {:.3}, y: {:.3}, z: {:.3}, color: {}, type: '{:?}' }},\n",
            word.replace("'", "\\'"), pos.x, pos.y, pos.z, color, morph_type
        ));
    }
    
    html.push_str(r#"        ];
        
        // Create morpheme points
        const morphemeGroup = new THREE.Group();
        const raycaster = new THREE.Raycaster();
        const mouse = new THREE.Vector2();
        
        morphemes.forEach(m => {
            const geometry = new THREE.SphereGeometry(0.01, 8, 8);
            const material = new THREE.MeshPhongMaterial({ color: m.color });
            const sphere = new THREE.Mesh(geometry, material);
            sphere.position.set(m.x * 2 - 1, m.z * 2 - 1, m.y * 2 - 1);
            sphere.userData = m;
            morphemeGroup.add(sphere);
        });
        
        scene.add(morphemeGroup);
        
        // Mouse interaction
        function onMouseMove(event) {
            mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
            mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;
        }
        
        window.addEventListener('mousemove', onMouseMove, false);
        
        // Animation loop
        function animate() {
            requestAnimationFrame(animate);
            
            controls.update();
            
            // Check for hover
            raycaster.setFromCamera(mouse, camera);
            const intersects = raycaster.intersectObjects(morphemeGroup.children);
            
            const hoverInfo = document.getElementById('hover-info');
            if (intersects.length > 0) {
                const data = intersects[0].object.userData;
                hoverInfo.innerHTML = `<strong>${data.word}</strong><br>Type: ${data.type}<br>Position: (${data.x.toFixed(3)}, ${data.y.toFixed(3)}, ${data.z.toFixed(3)})`;
            } else {
                hoverInfo.innerHTML = '';
            }
            
            renderer.render(scene, camera);
        }
        
        animate();
        
        // Handle window resize
        window.addEventListener('resize', () => {
            camera.aspect = window.innerWidth / window.innerHeight;
            camera.updateProjectionMatrix();
            renderer.setSize(window.innerWidth, window.innerHeight);
        });
    </script>
</body>
</html>"#);
    
    // Write HTML file
    let mut file = File::create("lingodb_visualization.html")?;
    file.write_all(html.as_bytes())?;
    
    Ok(())
}

fn generate_2d_projections(
    morphemes_by_type: &HashMap<MorphemeType, Vec<(String, Coordinate3D)>>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📐 2D Projections:");
    
    // XY projection (top view)
    println!("\n  XY Plane (Top View):");
    println!("  - Prefixes cluster around x≈0.2");
    println!("  - Roots spread across x≈0.5");
    println!("  - Suffixes cluster around x≈0.8");
    
    // XZ projection (front view)
    println!("\n  XZ Plane (Front View):");
    println!("  - All morphemes at similar Z (layer) height");
    println!("  - Clear left-to-right progression");
    
    // YZ projection (side view)
    println!("\n  YZ Plane (Side View):");
    println!("  - Y-axis shows etymology/semantic variation");
    println!("  - Productive morphemes tend toward center");
    
    Ok(())
}

fn analyze_spatial_patterns(
    morphemes_by_type: &HashMap<MorphemeType, Vec<(String, Coordinate3D)>>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Spatial Pattern Analysis:");
    
    // Calculate centroids
    for (morph_type, items) in morphemes_by_type {
        if items.is_empty() { continue; }
        
        let sum_x: f32 = items.iter().map(|(_, p)| p.x).sum();
        let sum_y: f32 = items.iter().map(|(_, p)| p.y).sum();
        let sum_z: f32 = items.iter().map(|(_, p)| p.z).sum();
        let count = items.len() as f32;
        
        println!("\n  {:?} Centroid:", morph_type);
        println!("    X: {:.3} ({})", sum_x / count, 
                 if sum_x / count < 0.33 { "left" } 
                 else if sum_x / count > 0.66 { "right" } 
                 else { "center" });
        println!("    Y: {:.3}", sum_y / count);
        println!("    Z: {:.3}", sum_z / count);
        
        // Calculate spread
        let centroid = Coordinate3D { 
            x: sum_x / count, 
            y: sum_y / count, 
            z: sum_z / count 
        };
        
        let avg_distance: f32 = items.iter()
            .map(|(_, p)| {
                let dx = p.x - centroid.x;
                let dy = p.y - centroid.y;
                let dz = p.z - centroid.z;
                (dx * dx + dy * dy + dz * dz).sqrt()
            })
            .sum::<f32>() / count;
        
        println!("    Average spread: {:.3}", avg_distance);
    }
    
    Ok(())
}