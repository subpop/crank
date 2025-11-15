-- Basic test example using LuaUnit
-- See https://github.com/bluebird75/luaunit for more information

-- Load LuaUnit
local luaunit = require('luaunit')

-- Test functions should start with 'test' or 'Test'
function testBasicAssertion()
    luaunit.assertEquals(1 + 1, 2)
end

function testStringOperations()
    local str = "Hello, Playdate!"
    luaunit.assertIsString(str)
    luaunit.assertTrue(#str > 0)
end

function testTableOperations()
    local t = {a = 1, b = 2, c = 3}
    luaunit.assertIsTable(t)
    luaunit.assertEquals(t.a, 1)
    luaunit.assertNotEquals(t.b, 3)
end

function testMathOperations()
    luaunit.assertEquals(2 * 2, 4)
    luaunit.assertTrue(5 > 3)
    luaunit.assertFalse(1 > 10)
end

-- Run all tests
-- This will automatically discover and run all functions starting with 'test'
os.exit(luaunit.LuaUnit.run())

