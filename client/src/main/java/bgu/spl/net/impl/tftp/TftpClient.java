package bgu.spl.net.impl.tftp;

import bgu.spl.net.impl.srv.BaseClient;

public class TftpClient {

    public static void main(String[] args) {
        if (args.length==0){
            Util.runningOnLinux = false;
            args = new String[]{"127.0.0.1", "7777"};
        }
        if (args.length != 2) {
            System.out.println("you must supply two argument: host and port");
            System.exit(1);
        }

        BaseClient baseClient = new BaseClient(Integer.parseInt(args[1]),
                new TftpClientProtocol(), new TftpEncoderDecoder());
        baseClient.consume(args[0]);
    }
}
