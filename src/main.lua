--label:配置
--information:https://github.com/sevenc-nanashi/aviutl2-scripts/blob/main/scripts/%E3%83%89%E3%83%83%E3%83%88%E7%B5%B5%E5%A4%89%E5%BD%A2.anm2

---$string:対象文字
local target_chars = ""

---$check:対象文字の反転
local invert_target = false

---$check:正規表現
local regex = false

--group:座標,false

---$track:X移動量
---min=-100000
---max=100000
---step=0.01
local dx = 0

---$track:Y移動量
---min=-100000
---max=100000
---step=0.01
local dy = 0

---$track:Z移動量
---min=-100000
---max=100000
---step=0.01
local dz = 0

--group:中心,false

---$track:中心X
---min=-5000
---max=5000
---step=0.01
local center_x = 0

---$track:中心Y
---min=-5000
---max=5000
---step=0.01
local center_y = 0

---$track:中心Z
---min=-5000
---max=5000
---step=0.01
local center_z = 0

--group:回転,false

---$track:X軸回転
---min=-3600
---max=3600
---step=0.01
local angle_x = 0

---$track:Y軸回転
---min=-3600
---max=3600
---step=0.01
local angle_y = 0

---$track:Z軸回転
---min=-3600
---max=3600
---step=0.01
local angle_z = 0

--group:拡大率,false

---$track:拡大率
---min=0
---max=10000
---step=0.001
local zoom = 100

---$track:X拡大率
---min=-10000
---max=10000
---step=0.001
local scale_x = 100

---$track:Y拡大率
---min=-10000
---max=10000
---step=0.001
local scale_y = 100

---$track:Z拡大率
---min=-10000
---max=10000
---step=0.001
local scale_z = 100

zoom = zoom / 100
scale_x = scale_x / 100
scale_y = scale_y / 100
scale_z = scale_z / 100

--group:色,false

---$track:透明度
---min=0
---max=100
---step=0.01
local transparency = 0

transparency = transparency / 100

---$color:文字色
local color = nil

--group:その他,false

---$check:エフェクト終端
local terminate = false

--group:高度な設定,false

---$check:デバッグモード
local debug = false

---$value:PI
local PI = {}

-- PIからパラメータを取得
if type(PI.target_chars) == "string" then
    target_chars = PI.target_chars
end
if type(PI.invert_target) == "boolean" then
    invert_target = PI.invert_target
end
if type(PI.regex) == "boolean" then
    regex = PI.regex
end
if type(PI.dx) == "number" then
    dx = PI.dx
end
if type(PI.dy) == "number" then
    dy = PI.dy
end
if type(PI.dz) == "number" then
    dz = PI.dz
end
if type(PI.center_x) == "number" then
    center_x = PI.center_x
end
if type(PI.center_y) == "number" then
    center_y = PI.center_y
end
if type(PI.center_z) == "number" then
    center_z = PI.center_z
end
if type(PI.angle_x) == "number" then
    angle_x = PI.angle_x
end
if type(PI.angle_y) == "number" then
    angle_y = PI.angle_y
end
if type(PI.angle_z) == "number" then
    angle_z = PI.angle_z
end
if type(PI.zoom) == "number" then
    zoom = PI.zoom
end
if type(PI.scale_x) == "number" then
    scale_x = PI.scale_x
end
if type(PI.scale_y) == "number" then
    scale_y = PI.scale_y
end
if type(PI.scale_z) == "number" then
    scale_z = PI.scale_z
end
if type(PI.transparency) == "number" then
    transparency = PI.transparency
end
if type(PI.color) == "number" then
    color = PI.color
elseif type(PI.color) == "boolean" and not PI.color then
    color = nil
end
if type(PI.terminate) == "boolean" then
    terminate = PI.terminate
end
if type(PI.debug) == "boolean" then
    debug = PI.debug
end

-- デバッグ用関数
local function debug_dump_internal(o)
    if type(o) == "table" then
        local s = "{ "
        local keys = {}
        local is_array = true
        local max_index = 0
        for k, _ in pairs(o) do
            table.insert(keys, k)
            if type(k) ~= "number" or k < 1 or math.floor(k) ~= k then
                is_array = false
            else
                if k > max_index then
                    max_index = k
                end
            end
        end
        if is_array then
            table.sort(keys, function(a, b)
                return a < b
            end)
        else
            table.sort(keys, function(a, b)
                return tostring(a) < tostring(b)
            end)
        end
        for i, k in ipairs(keys) do
            local v = o[k]
            if i > 1 then
                s = s .. ", "
            end
            if is_array then
                s = s .. debug_dump_internal(v)
            else
                s = s .. tostring(k) .. " = " .. debug_dump_internal(v)
            end
        end

        return s .. " }"
    elseif type(o) == "string" then
        return string.format("%q", o)
    else
        return tostring(o)
    end
end
local function debug_dump(m, o)
    if debug then
        if o == nil then
            debug_print(debug_dump_internal(m))
        else
            debug_print(m .. ": " .. debug_dump_internal(o))
        end
    end
end

local internal = obj.module("transform_specific_chars")

local object_per_char = obj.getvalue("テキスト", "文字毎に個別オブジェクト")
if type(object_per_char) ~= "string" then
    error("テキストオブジェクト以外のオブジェクトに対してスクリプトが実行されました")
elseif object_per_char ~= "1" then
    error("「文字毎に個別オブジェクト」を有効にしてください")
end

local text = obj.getvalue("テキスト", "テキスト")
if obj.index == 0 then
    debug_dump("raw text", text)
end

if debug then
    internal.verify_char_parse(text, obj.num)
end

local is_target_char = internal.is_target_char(obj.index, regex, text, target_chars)
if invert_target then
    is_target_char = not is_target_char
end

if is_target_char then
    obj.ox = obj.ox + dx
    obj.oy = obj.oy + dy
    obj.oz = obj.oz + dz
    obj.rx = obj.rx + angle_x
    obj.ry = obj.ry + angle_y
    obj.rz = obj.rz + angle_z
    obj.cx = obj.cx + center_x
    obj.cy = obj.cy + center_y
    obj.cz = obj.cz + center_z
    obj.zoom = obj.zoom * zoom
    obj.sx = obj.sx * scale_x
    obj.sy = obj.sy * scale_y
    obj.sz = obj.sz * scale_z
    obj.alpha = obj.alpha * (1 - transparency)
    if color then
        obj.effect("単色化", "強さ", 100, "色", color, "輝度を保持する", "0")
    end
    if terminate then
        obj.draw()
    end
end