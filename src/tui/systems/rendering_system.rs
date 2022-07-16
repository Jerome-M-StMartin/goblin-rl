//Jerome M. St.Martin
//June 15, 2022


//-----------------------------------------------------------------------------
//----------------------------- Rendering System ------------------------------
//-----------------------------------------------------------------------------

use specs::{System, SystemData};

pub struct RenderingSystem {}

impl <'a> System<'a> for RenderingSystem {
    type SystemData = ( Entities<'a>,
                        ReadStorage<'a, Renderable>,
                        ReadStorage<'a, Position> );

    /* Example run() call:
     * let rs = RenderingSystem {};
     * let ecs_borrow: &specs::World = &self ecs;
     * let entities = ecs_ap
     *                .req_access(AccessKey::Entity)
     *                .read_storage<Entity>(&ecs_borrow);
     * 
     * let renderables = ecs_ap
     *                   .req_access(AccessKey::Renderable)
     *                   .read_storage<Renderable>(&ecs_borrow);
     *
     * let positions = ecs_ap
     *                 .req_access(AccessKey::Position)
     *                 .read_storage<Position>(&ecs_borrow);
     *
     * let rs_data = (entities, renderables, positions);
     * rs.run(rs_data);
     */

    fn run(&mut self, data: Self::SystemData) {
        let (entities, renderables, positions) = data;

        for (entity, renderable, position) in data.join() {

        }
    }
}
