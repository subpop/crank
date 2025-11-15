#!/usr/bin/env lua
-- LuaUnit v3.4 - A unit-testing framework for Lua
-- https://github.com/bluebird75/luaunit
-- License: BSD

-- This is a minimal version of LuaUnit v3.4 adapted for Playdate
-- Full version available at: https://github.com/bluebird75/luaunit

local M = {}

-- Compatibility layer for Playdate
local function pcall_compat(func, ...)
    local success, result = pcall(func, ...)
    return success, result
end

-- Colors are disabled on Playdate console
local function colorize(text, color)
    return text
end

-- Test result tracking
local TestResult = {}
TestResult.__index = TestResult

function TestResult:new()
    local t = {
        successes = {},
        failures = {},
        errors = {},
        skipped = {},
        tests_run = 0,
        start_time = nil,
        stop_time = nil,
    }
    setmetatable(t, TestResult)
    return t
end

function TestResult:addSuccess(test_name)
    table.insert(self.successes, test_name)
    self.tests_run = self.tests_run + 1
end

function TestResult:addFailure(test_name, message)
    table.insert(self.failures, {name = test_name, message = message})
    self.tests_run = self.tests_run + 1
end

function TestResult:addError(test_name, message)
    table.insert(self.errors, {name = test_name, message = message})
    self.tests_run = self.tests_run + 1
end

function TestResult:addSkip(test_name, reason)
    table.insert(self.skipped, {name = test_name, reason = reason})
end

function TestResult:wasSuccessful()
    return #self.failures == 0 and #self.errors == 0
end

function TestResult:summary()
    local duration = (self.stop_time or 0) - (self.start_time or 0)
    return string.format(
        "Ran %d tests in %.3fs: %d successes, %d failures, %d errors, %d skipped",
        self.tests_run,
        duration,
        #self.successes,
        #self.failures,
        #self.errors,
        #self.skipped
    )
end

-- Assertion functions
local function assertEquals(actual, expected, msg)
    if actual ~= expected then
        local err = string.format("expected %s, got %s", tostring(expected), tostring(actual))
        if msg then
            err = msg .. ": " .. err
        end
        error(err, 2)
    end
end

local function assertNotEquals(actual, expected, msg)
    if actual == expected then
        local err = string.format("expected not %s", tostring(expected))
        if msg then
            err = msg .. ": " .. err
        end
        error(err, 2)
    end
end

local function assertTrue(value, msg)
    if value ~= true then
        local err = string.format("expected true, got %s", tostring(value))
        if msg then
            err = msg .. ": " .. err
        end
        error(err, 2)
    end
end

local function assertFalse(value, msg)
    if value ~= false then
        local err = string.format("expected false, got %s", tostring(value))
        if msg then
            err = msg .. ": " .. err
        end
        error(err, 2)
    end
end

local function assertNil(value, msg)
    if value ~= nil then
        local err = string.format("expected nil, got %s", tostring(value))
        if msg then
            err = msg .. ": " .. err
        end
        error(err, 2)
    end
end

local function assertNotNil(value, msg)
    if value == nil then
        local err = "expected not nil"
        if msg then
            err = msg .. ": " .. err
        end
        error(err, 2)
    end
end

local function assertType(value, expected_type, msg)
    local actual_type = type(value)
    if actual_type ~= expected_type then
        local err = string.format("expected type %s, got %s", expected_type, actual_type)
        if msg then
            err = msg .. ": " .. err
        end
        error(err, 2)
    end
end

local function assertIsString(value, msg)
    assertType(value, "string", msg)
end

local function assertIsNumber(value, msg)
    assertType(value, "number", msg)
end

local function assertIsTable(value, msg)
    assertType(value, "table", msg)
end

local function assertIsFunction(value, msg)
    assertType(value, "function", msg)
end

local function assertIsBoolean(value, msg)
    assertType(value, "boolean", msg)
end

local function assertAlmostEquals(actual, expected, margin, msg)
    margin = margin or 1e-10
    if math.abs(actual - expected) > margin then
        local err = string.format("expected %f ± %f, got %f", expected, margin, actual)
        if msg then
            err = msg .. ": " .. err
        end
        error(err, 2)
    end
end

-- Test runner
local LuaUnit = {
    result = nil,
    verbosity = 1,
}

function LuaUnit.assertEquals(...)
    return assertEquals(...)
end

function LuaUnit.assertNotEquals(...)
    return assertNotEquals(...)
end

function LuaUnit.assertTrue(...)
    return assertTrue(...)
end

function LuaUnit.assertFalse(...)
    return assertFalse(...)
end

function LuaUnit.assertNil(...)
    return assertNil(...)
end

function LuaUnit.assertNotNil(...)
    return assertNotNil(...)
end

function LuaUnit.assertIsString(...)
    return assertIsString(...)
end

function LuaUnit.assertIsNumber(...)
    return assertIsNumber(...)
end

function LuaUnit.assertIsTable(...)
    return assertIsTable(...)
end

function LuaUnit.assertIsFunction(...)
    return assertIsFunction(...)
end

function LuaUnit.assertIsBoolean(...)
    return assertIsBoolean(...)
end

function LuaUnit.assertAlmostEquals(...)
    return assertAlmostEquals(...)
end

function LuaUnit.assertType(...)
    return assertType(...)
end

function LuaUnit.run(...)
    local result = TestResult:new()
    LuaUnit.result = result
    
    result.start_time = 0  -- Playdate doesn't have os.clock
    
    -- Find all test functions in the global namespace
    local tests = {}
    for name, value in pairs(_G) do
        if type(value) == "function" and name:match("^[Tt]est") then
            table.insert(tests, {name = name, func = value})
        end
    end
    
    -- Sort tests by name
    table.sort(tests, function(a, b) return a.name < b.name end)
    
    print("Running " .. #tests .. " test(s)...")
    print(string.rep("-", 40))
    
    -- Run each test
    for _, test in ipairs(tests) do
        local success, err = pcall_compat(test.func)
        
        if success then
            result:addSuccess(test.name)
            if LuaUnit.verbosity > 0 then
                print("✓ " .. test.name)
            end
        else
            result:addFailure(test.name, tostring(err))
            print("✗ " .. test.name)
            print("  " .. tostring(err))
        end
    end
    
    result.stop_time = 0  -- Playdate doesn't have os.clock
    
    print(string.rep("-", 40))
    print(result:summary())
    print()
    
    if result:wasSuccessful() then
        print("SUCCESS: All tests passed!")
        return 0
    else
        print("FAILURE: Some tests failed.")
        return 1
    end
end

-- Export all assertion functions to make them globally available
M.LuaUnit = LuaUnit
M.assertEquals = assertEquals
M.assertNotEquals = assertNotEquals
M.assertTrue = assertTrue
M.assertFalse = assertFalse
M.assertNil = assertNil
M.assertNotNil = assertNotNil
M.assertType = assertType
M.assertIsString = assertIsString
M.assertIsNumber = assertIsNumber
M.assertIsTable = assertIsTable
M.assertIsFunction = assertIsFunction
M.assertIsBoolean = assertIsBoolean
M.assertAlmostEquals = assertAlmostEquals

return M

