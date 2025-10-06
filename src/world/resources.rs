use bevy::prelude::*;
use super::data::RoomTypeData;

#[derive(Resource)]
pub struct WorldState {
    pub rooms: Vec<RoomTypeData>,
    pub current_room_index: usize,
    pub total_rooms: usize,
}

impl WorldState {
    pub fn new() -> Self {
        let rooms = vec![
            RoomTypeData::IntroRoom,
            RoomTypeData::FloorRoom,
            RoomTypeData::Pantry1,
            RoomTypeData::LibraryRoom,
            RoomTypeData::DiningHall,
            RoomTypeData::Laboratory,
            RoomTypeData::Corridor,
            RoomTypeData::FinalRoom,
        ];
        let total = rooms.len();
        Self {
            rooms,
            current_room_index: 0,
            total_rooms: total,
        }
    }
    
    pub fn current_room(&self) -> RoomTypeData {
        self.rooms[self.current_room_index]
    }
    
    pub fn advance(&mut self) -> bool {
        if self.has_next_room() {
            self.current_room_index += 1;
            true
        } else {
            false
        }
    }
    
    pub fn has_next_room(&self) -> bool {
        self.current_room_index < self.total_rooms - 1
    }
    
    pub fn progress(&self) -> String {
        format!("Raum {} von {}", self.current_room_index + 1, self.total_rooms)
    }
}