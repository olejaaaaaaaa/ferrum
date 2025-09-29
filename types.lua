---@class Vector3
---@field x number
---@field y number  
---@field z number

---@class Entity
---@field name string
---@field position Vector3
---@field health number
---@field move fun(self: Entity, x: number, y: number, z: number)
---@field take_damage fun(self: Entity, damage: number): number

---Создает сущность
---@param name string
---@param x number
---@param y number
---@param z number
---@return Entity
function create_entity(name, x, y, z) end

---Играет звук
---@param path string
---@param volume number
function play_sound(path, volume) end

---Дельта времени
---@type number
delta_time = 0.0