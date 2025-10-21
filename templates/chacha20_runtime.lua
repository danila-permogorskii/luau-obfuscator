-- Pure Luau ChaCha20 Implementation
-- Optimized for Roblox sandbox (uses only bit32 library)
-- Compatible with Luau JIT

local ChaCha20 = {}

-- Bit32 operations (available in Roblox)
local band, bor, bxor = bit32.band, bit32.bor, bit32.bxor
local lrotate, rshift, lshift = bit32.lrotate, bit32.rshift, bit32.lshift

-- Convert 4 bytes to uint32 (little-endian)
local function bytes_to_uint32(b1, b2, b3, b4)
    return bor(
        b1,
        lshift(b2, 8),
        lshift(b3, 16),
        lshift(b4, 24)
    )
end

-- Convert uint32 to 4 bytes (little-endian)
local function uint32_to_bytes(n)
    return 
        band(n, 0xFF),
        band(rshift(n, 8), 0xFF),
        band(rshift(n, 16), 0xFF),
        band(rshift(n, 24), 0xFF)
end

-- ChaCha20 quarter round
local function quarter_round(state, a, b, c, d)
    -- a += b; d ^= a; d <<<= 16
    state[a] = band(state[a] + state[b], 0xFFFFFFFF)
    state[d] = bxor(state[d], state[a])
    state[d] = lrotate(state[d], 16)
    
    -- c += d; b ^= c; b <<<= 12
    state[c] = band(state[c] + state[d], 0xFFFFFFFF)
    state[b] = bxor(state[b], state[c])
    state[b] = lrotate(state[b], 12)
    
    -- a += b; d ^= a; d <<<= 8
    state[a] = band(state[a] + state[b], 0xFFFFFFFF)
    state[d] = bxor(state[d], state[a])
    state[d] = lrotate(state[d], 8)
    
    -- c += d; b ^= c; b <<<= 7
    state[c] = band(state[c] + state[d], 0xFFFFFFFF)
    state[b] = bxor(state[b], state[c])
    state[b] = lrotate(state[b], 7)
end

-- ChaCha20 block function - generates 64 bytes of keystream
function ChaCha20.block(key, nonce, counter)
    -- Initialize state (16 x 32-bit words = 64 bytes)
    local state = {
        -- Constants "expand 32-byte k"
        0x61707865, 0x3320646e, 0x79622d32, 0x6b206574,
        -- 256-bit key (8 words)
        key[1], key[2], key[3], key[4],
        key[5], key[6], key[7], key[8],
        -- Counter (1 word)
        counter,
        -- Nonce (3 words)
        nonce[1], nonce[2], nonce[3]
    }
    
    -- Save initial state
    local initial = {}
    for i = 1, 16 do
        initial[i] = state[i]
    end
    
    -- 20 rounds (10 double rounds)
    for i = 1, 10 do
        -- Column rounds
        quarter_round(state, 1, 5, 9, 13)
        quarter_round(state, 2, 6, 10, 14)
        quarter_round(state, 3, 7, 11, 15)
        quarter_round(state, 4, 8, 12, 16)
        
        -- Diagonal rounds
        quarter_round(state, 1, 6, 11, 16)
        quarter_round(state, 2, 7, 12, 13)
        quarter_round(state, 3, 8, 9, 14)
        quarter_round(state, 4, 5, 10, 15)
    end
    
    -- Add initial state
    for i = 1, 16 do
        state[i] = band(state[i] + initial[i], 0xFFFFFFFF)
    end
    
    -- Convert to byte array
    local output = {}
    for i = 1, 16 do
        local b1, b2, b3, b4 = uint32_to_bytes(state[i])
        table.insert(output, b1)
        table.insert(output, b2)
        table.insert(output, b3)
        table.insert(output, b4)
    end
    
    return output
end

-- Encrypt/decrypt data (XOR with keystream)
function ChaCha20.crypt(key, nonce, plaintext)
    local output = {}
    local counter = 0
    local pos = 1
    
    while pos <= #plaintext do
        -- Generate next block
        local keystream = ChaCha20.block(key, nonce, counter)
        
        -- XOR plaintext with keystream
        for i = 1, math.min(64, #plaintext - pos + 1) do
            output[pos] = bxor(plaintext[pos], keystream[i])
            pos = pos + 1
        end
        
        counter = counter + 1
    end
    
    return output
end

-- Base64 decode utility (for embedded encrypted data)
local base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
local base64_decode_table = {}
for i = 1, #base64_chars do
    base64_decode_table[base64_chars:sub(i, i)] = i - 1
end

function ChaCha20.base64_decode(str)
    local output = {}
    local len = #str
    local i = 1
    
    while i <= len do
        local c1 = base64_decode_table[str:sub(i, i)] or 0
        local c2 = base64_decode_table[str:sub(i+1, i+1)] or 0
        local c3 = base64_decode_table[str:sub(i+2, i+2)] or 0
        local c4 = base64_decode_table[str:sub(i+3, i+3)] or 0
        
        local n = bor(lshift(c1, 18), lshift(c2, 12), lshift(c3, 6), c4)
        
        table.insert(output, band(rshift(n, 16), 0xFF))
        if str:sub(i+2, i+2) ~= "=" then
            table.insert(output, band(rshift(n, 8), 0xFF))
        end
        if str:sub(i+3, i+3) ~= "=" then
            table.insert(output, band(n, 0xFF))
        end
        
        i = i + 4
    end
    
    return output
end

-- Byte array to string
function ChaCha20.bytes_to_string(bytes)
    local chars = {}
    for i = 1, #bytes do
        chars[i] = string.char(bytes[i])
    end
    return table.concat(chars)
end

-- String to byte array
function ChaCha20.string_to_bytes(str)
    local bytes = {}
    for i = 1, #str do
        bytes[i] = str:byte(i)
    end
    return bytes
end

-- Parse key from base64 string (32 bytes = 8 uint32)
function ChaCha20.parse_key(key_b64)
    local key_bytes = ChaCha20.base64_decode(key_b64)
    local key = {}
    for i = 0, 7 do
        key[i+1] = bytes_to_uint32(
            key_bytes[i*4 + 1],
            key_bytes[i*4 + 2],
            key_bytes[i*4 + 3],
            key_bytes[i*4 + 4]
        )
    end
    return key
end

-- Parse nonce from base64 string (12 bytes = 3 uint32)
function ChaCha20.parse_nonce(nonce_b64)
    local nonce_bytes = ChaCha20.base64_decode(nonce_b64)
    local nonce = {}
    for i = 0, 2 do
        nonce[i+1] = bytes_to_uint32(
            nonce_bytes[i*4 + 1],
            nonce_bytes[i*4 + 2],
            nonce_bytes[i*4 + 3],
            nonce_bytes[i*4 + 4]
        )
    end
    return nonce
end

-- High-level decrypt function
function ChaCha20.decrypt_string(encrypted_b64, key_b64, nonce_b64)
    local key = ChaCha20.parse_key(key_b64)
    local nonce = ChaCha20.parse_nonce(nonce_b64)
    local ciphertext = ChaCha20.base64_decode(encrypted_b64)
    local plaintext = ChaCha20.crypt(key, nonce, ciphertext)
    return ChaCha20.bytes_to_string(plaintext)
end

return ChaCha20
