package bgu.spl.net.impl.tftp;

import java.io.File;
import java.io.FileOutputStream;
import java.io.FileInputStream;
import java.io.IOException;
import java.nio.file.Files;
import java.util.List;

public class Util {
    public static final short MAX_PACKET_LENGTH = 512;
    public static boolean runningOnLinux = true;

    public static byte[] convertListToArray(List<Byte> list) {
        byte[] retByte = new byte[list.size()];
        for (int i = 0; i < list.size(); i++) {
            retByte[i] = list.get(i);
        }
        return retByte;
    }

    public static byte[] convertShortToByteArray(short s) {
        return new byte[]{(byte) ((s >> 8) & 0xFF), (byte) (s & 0xff)};
    }

    public static short convertBytesToShort(byte byte1, byte byte2) {
        return (short) (((byte1 & 0xFF) << 8) | (byte2 & 0xFF));
    }

    public static byte[] createDataPacket(short blockNumber, byte[] message) {
        return concatArrays(new byte[]{0, 3},
                convertShortToByteArray((short) message.length),
                convertShortToByteArray(blockNumber),
                message);
    }

    public static byte[] getPartOfArray(byte[] arr, int startIndex, int lastIndex) {
        byte[] result = new byte[lastIndex - startIndex + 1];
        System.arraycopy(arr, startIndex, result, 0, result.length);
        return result;
    }

    public static byte[] getPartOfArray(byte[] arr, int startIndex) {
        return getPartOfArray(arr, startIndex, arr.length - 1);
    }

    public static byte[] concatArrays(byte[]... arrays) {
        int totalLength = 0;
        for (byte[] array : arrays) {
            totalLength += array.length;
        }
        byte[] result = new byte[totalLength];
        int currentIndex = 0;
        for (byte[] array : arrays) {
            System.arraycopy(array, 0, result, currentIndex, array.length);
            currentIndex += array.length;
        }
        return result;
    }

    public static File getFilesDirectory() {
        String path;
        if (!runningOnLinux)
            path = System.getProperty("user.dir") + "\\client\\src\\main\\java\\bgu\\spl\\net\\impl\\tftp\\Files";
        else path = "src/main/java/bgu/spl/net/impl/tftp/Files/";
        return new File(path);
    }

    public static File getFile(String fileName) {
        File[] arr = getFilesDirectory().listFiles((dir, name) -> name.equals(fileName));
        if (arr == null || arr.length == 0) return new File(getFilesDirectory(), fileName);
        return arr[0];
    }

    public static boolean isExists(String filename) {
        File directory = getFilesDirectory();
        File[] files = directory.listFiles();
        if (files == null) return false;
        for (File f : files) {
            if (f.getName().equals(filename)) return true;
        }
        return false;
    }

    public static void writeInto(File destination, byte[] data) throws IOException {
        FileOutputStream out = new FileOutputStream(destination, true);
        out.write(data);
        out.close();
    }

    public static byte[] readPartOfFile(File file, short part) throws IOException {
        return readPartOfFile(file, (long) (part - 1) * MAX_PACKET_LENGTH, MAX_PACKET_LENGTH);
    }

    private static byte[] readPartOfFile(File file, long startPosition, short bytesToRead) throws IOException {
        long fileSize = Files.size(file.toPath());
        if (bytesToRead > fileSize - startPosition)
            bytesToRead = (short) (fileSize - startPosition);
        //prevent error of reading outside the file
        if (bytesToRead < 1) return null;
        try (FileInputStream fis = new FileInputStream(file)) {
            fis.skip(startPosition); // Move to the start position
            byte[] buffer = new byte[bytesToRead];
            int bytesRead = fis.read(buffer); // Read bytes into the buffer
            if (bytesRead != -1) return buffer;
        }
        return null;
    }

    public static byte[] getError(int errorType) {
        if (errorType < 0 | errorType > 9) throw new IllegalArgumentException("Illegal error type inserted!");
        byte[] error = new byte[]{0, 5, 0, (byte) errorType};
        String errorMessage;

        switch (errorType) {
            case 1:
                errorMessage = "File not found – RRQ / DELRQ of non-existing file.";
                break;
            case 2:
                errorMessage = "Access violation – File cannot be written, read or deleted.";
                break;
            case 3:
                errorMessage = "Disk full or allocation exceeded – No room in disk.";
                break;
            case 4:
                errorMessage = "Illegal TFTP operation – Unknown Opcode.";
                break;
            case 5:
                errorMessage = "File already exists – File name exists on WRQ.";
                break;
            case 6:
                errorMessage = "User not logged in – Any opcode received before Login completes.";
                break;
            case 7:
                errorMessage = "User already logged in – Login username already connected.";
                break;
            default:
                throw new IllegalArgumentException("Illegal error type inserted!");
        }
        return concatArrays(error, errorMessage.getBytes(), new byte[]{0});
    }

    public static enum OP {None, RRQ, WRQ, DATA, ACK, ERROR, DIRQ, LOGRQ, DELRQ, BCAST, DISC}

    public static OP getOpByByte(byte n) {
        if (n == 0) return OP.None;
        else if (n == 1) return OP.RRQ;
        else if (n == 2) return OP.WRQ;
        else if (n == 3) return OP.DATA;
        else if (n == 4) return OP.ACK;
        else if (n == 5) return OP.ERROR;
        else if (n == 6) return OP.DIRQ;
        else if (n == 7) return OP.LOGRQ;
        else if (n == 8) return OP.DELRQ;
        else if (n == 9) return OP.BCAST;
        else if (n == 0xa) return OP.DISC;
        throw new RuntimeException("not an OP code");
    }
}
