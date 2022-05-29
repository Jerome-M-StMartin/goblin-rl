:::mermaid
%% Key:
%% Channel_TX ..> Channel_RX
%% Mutator --|> Mutatee
%% Reader --> Readee
%% Owner --o Owned (Aggregation)



classDiagram

%% Classes

    class Main {
        -Thread model
        -Thread view
        -Controller controller
    }

    class Controller {
        -UserInput user_input

        +read_from(crossterm) InputEvent
        -event_to_cmd(InputEvent) Model_GameWorldCommand
        -send(tx, Model_GameWorldCommand) Result
    }

    class Model_GameWorld {
        -Map map
        -ECSWorld ecs
        
        +notify_map_mut(tx) : (&Map)
    }
        class Map {
            -Tile[] tile_map
            -int[] blocked_tiles
            -int[] bloodied_tiles
            -int[] visible_tiles

            +replace(Tile) Result
        }
            class Tile {
                -Point pos
                -Entity[] content
            }

        class ECS {
            -Storage entities
            -Storage components
            -fn systems()
        }

    class View_GUI {
        -Widget[] widgets

        +draw_map(MapDrawable)
        +draw_gui(GUI)
    }


%% Relationships


    %% Connected Across Thread Boundry
    User <.. View_GUI
    Model_GameWorld <..> View_GUI
    Controller ..> Model_GameWorld

    %% A mutates B directly (most stuff mutates self based on input)
    User --|> Crossterm
    View_GUI --|> Crossterm

    %% A reads from B (no mutation)
    Crossterm --|> Controller

    %% A owns B
    Map --o Tile
    Model_GameWorld --o Map
    Model_GameWorld --o ECS

    %% Threads
    View_Thread -- View_GUI
    Main --o Controller
    Main -- Model_Thread
    Main -- View_Thread
    Model_Thread -- Model_GameWorld
:::