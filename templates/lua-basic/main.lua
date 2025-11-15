import "CoreLibs/graphics"
import "CoreLibs/sprites"
import "CoreLibs/timer"

local gfx <const> = playdate.graphics

-- Game variables
local playerSprite = nil
local score = 0

function initGame()
    -- Set up the player sprite
    local playerImage = gfx.image.new(40, 40)
    gfx.pushContext(playerImage)
        gfx.fillCircleAtPoint(20, 20, 20)
    gfx.popContext()
    
    playerSprite = gfx.sprite.new(playerImage)
    playerSprite:moveTo(200, 120)
    playerSprite:add()
    
    score = 0
end

function playdate.update()
    -- Update sprites
    gfx.sprite.update()
    
    -- Handle input
    if playdate.buttonIsPressed(playdate.kButtonUp) then
        playerSprite:moveBy(0, -2)
    end
    if playdate.buttonIsPressed(playdate.kButtonDown) then
        playerSprite:moveBy(0, 2)
    end
    if playdate.buttonIsPressed(playdate.kButtonLeft) then
        playerSprite:moveBy(-2, 0)
    end
    if playdate.buttonIsPressed(playdate.kButtonRight) then
        playerSprite:moveBy(2, 0)
    end
    
    -- Keep player on screen
    local x, y = playerSprite:getPosition()
    x = math.max(20, math.min(380, x))
    y = math.max(20, math.min(220, y))
    playerSprite:moveTo(x, y)
    
    -- Draw UI
    gfx.drawText("Score: " .. score, 10, 10)
    
    -- Update timers
    playdate.timer.updateTimers()
end

-- Initialize the game
initGame()

