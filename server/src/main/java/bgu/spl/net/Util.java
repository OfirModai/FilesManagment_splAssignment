package bgu.spl.net;

import java.nio.charset.StandardCharsets;
import java.util.List;

public class Util {
    public static byte[] convertListToArray(List<Byte> list) {
        byte[] retByte = new byte[list.size()];
        for (int i = 0; i < list.size(); i++) {
            retByte[i] = list.get(i);
        }
        return retByte;
    }

    public static byte[] hexToBytes(byte[] hexBytes) {
        byte[] bytes = new byte[hexBytes.length / 2];
        for (int i = 0; i < hexBytes.length; i += 2) {
            String hexPair = new String(hexBytes, i, 2, StandardCharsets.US_ASCII);
            bytes[i / 2] = (byte) Integer.parseInt(hexPair, 16);
        }
        return bytes;
    }
}
