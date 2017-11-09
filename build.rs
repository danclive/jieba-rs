extern crate gcc;

fn main() {
    let mut c = gcc::Build::new();

    c.warnings(false);

    c.include("cppjieba/limonp");
    c.include("cppjieba");
    c.include("src");

    let files = &[
        "cppjieba/limonp/ArgvContext.hpp", "cppjieba/limonp/BoundedQueue.hpp",
        "cppjieba/limonp/Closure.hpp", "cppjieba/limonp/Colors.hpp", "cppjieba/limonp/Condition.hpp", "cppjieba/limonp/Config.hpp", "cppjieba/limonp/FileLock.hpp",
        "cppjieba/limonp/ForcePublic.hpp", "cppjieba/limonp/LocalVector.hpp", "cppjieba/limonp/Logging.hpp", "cppjieba/limonp/Md5.hpp",
        "cppjieba/limonp/MutexLock.hpp", "cppjieba/limonp/NonCopyable.hpp", "cppjieba/limonp/StdExtension.hpp", "cppjieba/limonp/StringUtil.hpp",
        "cppjieba/limonp/Thread.hpp", "cppjieba/limonp/ThreadPool.hpp", "cppjieba/limonp/BoundedBlockingQueue.hpp",

        "cppjieba/DictTrie.hpp", "cppjieba/FullSegment.hpp",
        "cppjieba/KeywordExtractor.hpp", "cppjieba/MPSegment.hpp", "cppjieba/MixSegment.hpp",
        "cppjieba/PreFilter.hpp", "cppjieba/QuerySegment.hpp", "cppjieba/SegmentBase.hpp",
        "cppjieba/TextRankExtractor.hpp", "cppjieba/Trie.hpp", "cppjieba/Unicode.hpp",
        "cppjieba/HMMModel.hpp",
        "cppjieba/PosTagger.hpp",
        "cppjieba/HMMSegment.hpp",
        "cppjieba/SegmentTagged.hpp",

        "cppjieba/Jieba.hpp",

        "src/util.c",
        "src/jieba.cpp",
    ];

    for file in files.iter() {
        c.file(file);
    }

    c.cpp(true);

    c.compile("cppjieba");
}
