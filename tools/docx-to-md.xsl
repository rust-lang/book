<?xml version="1.0"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w10="urn:schemas-microsoft-com:office:word" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml">
    <xsl:output method="text" />
    <xsl:template match="/">
        <xsl:apply-templates select="/w:document/w:body/*" />
    </xsl:template>

    <!-- Ignore these -->
    <xsl:template match="w:p[starts-with(w:pPr/w:pStyle/@w:val, 'TOC')]" />
    <xsl:template match="w:p[starts-with(w:pPr/w:pStyle/@w:val, 'Contents1')]" />
    <xsl:template match="w:p[starts-with(w:pPr/w:pStyle/@w:val, 'Contents2')]" />
    <xsl:template match="w:p[starts-with(w:pPr/w:pStyle/@w:val, 'Contents3')]" />

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'ChapterStart']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'ChapterNumber']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'Normal']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'Standard']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'AuthorQuery']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'BookHalfTitle']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'BookTitle0']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'BookEdition']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'BookAuthor']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'Copyright']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'CopyrightHead']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'CopyrightLOC']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'ProductionDirective0']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'ProductionDirective']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'BoxType']" />
    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'GraphicSlug']" />

    <xsl:template match="w:p[w:pPr[not(w:pStyle)]]" />

    <xsl:variable name="chapternumber" select="//w:p[w:pPr/w:pStyle/@w:val = 'ChapterNumber']/w:r/w:t" />
    <xsl:variable name="appendixnumber" select="//w:p[w:pPr/w:pStyle/@w:val = 'AppendixNumber']/w:r/w:t" />

    <!-- Paragraph styles -->

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'AppendixNumber']" >
        <xsl:text>&#10;[TOC]&#10;&#10;</xsl:text>
        <xsl:text>## Appendix </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>: </xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'AppendixTitle']">
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'ChapterTitle']">
        <xsl:text>&#10;[TOC]&#10;&#10;</xsl:text>
        <xsl:text># </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>


    <xsl:template match="w:p[(w:pPr/w:pStyle/@w:val = 'HeadA' or w:pPr/w:pStyle/@w:val = 'FrontmatterTitle') and w:r/w:t]">
        <xsl:text>## </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>
    <xsl:template match="w:p[(w:pPr/w:pStyle/@w:val = 'HeadA' or w:pPr/w:pStyle/@w:val = 'FrontmatterTitle') and not(w:r/w:t)]" />

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'HeadB']">
        <xsl:text>### </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'HeadC']">
        <xsl:text>#### </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'HeadBox']">
        <xsl:text>### </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'NumListA' or @w:val = 'NumListB' or @w:val = 'ListNumber' or @w:val = 'ListNumber0']]">
        <xsl:text>1. </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;</xsl:text>
        <xsl:if test="not(following-sibling::*[1][self::w:p]) or following-sibling::w:p[1][w:pPr/w:pStyle[@w:val != 'NumListA' and @w:val != 'NumListB' and @w:val != 'ListNumber' and @w:val != 'ListNumber0']]">
            <xsl:text>&#10;</xsl:text>
        </xsl:if>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'NumListC']]">
        <xsl:text>1. </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BulletA' or @w:val = 'BulletB' or @w:val = 'ListPlainA' or @w:val = 'ListPlainB' or @w:val = 'ListBullet' or @w:val = 'ListPlain' or @w:val = 'ListBullet0']]">
        <xsl:text>* </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;</xsl:text>
        <xsl:if test="not(following-sibling::*[1][self::w:p]) or following-sibling::w:p[1][w:pPr/w:pStyle[@w:val != 'BulletA' and @w:val != 'BulletB' and @w:val != 'ListPlainA' and @w:val != 'ListPlainB' and @w:val != 'ListBullet' and @w:val != 'ListPlain' and @w:val != 'ListBullet0']]">
            <xsl:text>&#10;</xsl:text>
        </xsl:if>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BoxListBullet']]">
        <xsl:text>> * </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;</xsl:text>
        <xsl:choose>
            <xsl:when test="following-sibling::w:p[1][w:pPr/w:pStyle[@w:val = 'BoxBody']]">
                <xsl:text>>&#10;</xsl:text>
            </xsl:when>
            <xsl:when test="not(following-sibling::*[1][self::w:p])">
                <xsl:text>&#10;</xsl:text>
            </xsl:when>
        </xsl:choose>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BulletC' or @w:val = 'ListPlainC']]">
        <xsl:text>* </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'SubBullet']]">
        <xsl:text>  * </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'CodeLabel']]">
        <xsl:text>Filename: </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BodyFirst' or @w:val = 'Body' or @w:val = 'BodyFirstBox' or @w:val = 'BodyBox' or @w:val = '1stPara' or @w:val = 'ChapterIntro' or @w:val = 'BodyContinued' or @w:val = 'SourceForeword']]">
        <xsl:if test=".//w:t">
            <xsl:apply-templates select="*" />
            <xsl:text>&#10;&#10;</xsl:text>
        </xsl:if>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'CodeA' or @w:val = 'CodeAWingding']]">
        <xsl:text>```&#10;</xsl:text>
        <!-- Don't apply Emphasis/etc templates in code blocks -->
        <xsl:for-each select="w:r">
            <xsl:value-of select="w:t" />
        </xsl:for-each>
        <xsl:text>&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'CodeB' or @w:val = 'CodeBWingding']]">
        <!-- Don't apply Emphasis/etc templates in code blocks -->
        <xsl:for-each select="w:r">
            <xsl:value-of select="w:t" />
        </xsl:for-each>
        <xsl:text>&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'CodeC' or @w:val = 'CodeCWingding']]">
        <!-- Don't apply Emphasis/etc templates in code blocks -->
        <xsl:for-each select="w:r">
            <xsl:value-of select="w:t" />
        </xsl:for-each>
        <xsl:text>&#10;```&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'Code' or @w:val = 'CodeWide' or @w:val = 'CodeAnnotated']]">
        <xsl:if test="not(preceding-sibling::*[1][self::w:p]) or preceding-sibling::w:p[1][w:pPr[not(w:pStyle) or (w:pStyle/@w:val != 'Code' and w:pStyle/@w:val != 'CodeWide' and w:pStyle/@w:val != 'CodeAnnotated')]]">
            <xsl:text>```&#10;</xsl:text>
        </xsl:if>

        <!-- Don't apply Emphasis/etc templates in code blocks -->
        <xsl:for-each select="w:r">
            <xsl:value-of select="w:t" />
        </xsl:for-each>

        <xsl:text>&#10;</xsl:text>

        <xsl:if test="not(following-sibling::*[1][self::w:p]) or following-sibling::w:p[1][w:pPr[not(w:pStyle) or (w:pStyle/@w:val != 'Code' and w:pStyle/@w:val != 'CodeWide' and w:pStyle/@w:val != 'CodeAnnotated')]]">
            <xsl:text>```&#10;&#10;</xsl:text>
        </xsl:if>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BoxCode']]">
        <xsl:choose>
            <xsl:when test="not(preceding-sibling::*[1][self::w:p]) or preceding-sibling::w:p[1][w:pPr[not(w:pStyle) or w:pStyle/@w:val != 'BoxCode']]">
                <xsl:text>> ```&#10;> </xsl:text>
            </xsl:when>
            <xsl:otherwise>
                <xsl:text>> </xsl:text>
            </xsl:otherwise>
        </xsl:choose>

        <xsl:apply-templates select="*" />

        <xsl:choose>
            <xsl:when test="following-sibling::w:p[1][w:pPr/w:pStyle/@w:val = 'BoxCode']">
                <xsl:text>&#10;</xsl:text>
            </xsl:when>
            <xsl:when test="following-sibling::w:p[1][w:pPr/w:pStyle/@w:val = 'BoxBody']">
                <xsl:text>&#10;> ```&#10;>&#10;</xsl:text>
            </xsl:when>
            <xsl:otherwise>
                <xsl:text>&#10;> ```&#10;&#10;</xsl:text>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'CodeSingle']">
        <xsl:text>```&#10;</xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;```&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'TableTitle']]">
        <xsl:text>Table </xsl:text>
        <xsl:value-of select="$chapternumber" />
        <xsl:value-of select="$appendixnumber" />
        <xsl:text>-</xsl:text>
        <xsl:number level="any" count="w:p[w:pPr/w:pStyle[@w:val = 'TableTitle']]" />
        <xsl:text>: </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'Caption' or @w:val = 'Caption1' or @w:val = 'Listing' or @w:val = 'CodeListingCaption']]">
        <xsl:text>Listing </xsl:text>
        <xsl:value-of select="$chapternumber" />
        <xsl:value-of select="$appendixnumber" />
        <xsl:text>-</xsl:text>
        <xsl:number level="any" count="w:p[w:pPr/w:pStyle[@w:val = 'Caption' or @w:val = 'Caption1' or @w:val = 'Listing' or @w:val = 'CodeListingCaption']]" />
        <xsl:text>: </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BlockQuote' or @w:val = 'QuotePara']]">
        <xsl:text>> </xsl:text>
        <xsl:apply-templates select="*" />
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BoxTitle']]">
        <xsl:text>> ### </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;>&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle[@w:val = 'BlockText' or @w:val = 'BoxBody']]">
        <xsl:text>> </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:choose>
            <xsl:when test="following-sibling::w:p[1][w:pPr/w:pStyle/@w:val = 'BlockText' or w:pPr/w:pStyle/@w:val = 'BoxBody' or w:pPr/w:pStyle/@w:val = 'BoxListBullet' or w:pPr/w:pStyle/@w:val = 'BoxCode' or w:pPr/w:pStyle/@w:val = 'BoxRunInHead']">
                <xsl:text>&#10;>&#10;</xsl:text>
            </xsl:when>
            <xsl:otherwise>
                <xsl:text>&#10;&#10;</xsl:text>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'Note']">
        <xsl:text>> Note: </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>
    <xsl:template match="w:r[w:rPr/w:rStyle/@w:val = 'NoteHead']" />

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'RunInHead']">
        <xsl:text>* **</xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>**: </xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'RunInPara']">
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;</xsl:text>
        <xsl:if test="following-sibling::w:p[1][w:pPr/w:pStyle/@w:val != 'RunInHead']">
            <xsl:text>&#10;</xsl:text>
        </xsl:if>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'BoxRunInHead']">
        <xsl:text>> * **</xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>**: </xsl:text>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'BoxRunInPara']">
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;</xsl:text>
        <xsl:if test="following-sibling::w:p[1][w:pPr/w:pStyle/@w:val != 'BoxRunInHead']">
            <xsl:text>>&#10;</xsl:text>
        </xsl:if>
    </xsl:template>

    <xsl:template match="w:p[w:pPr/w:pStyle/@w:val = 'CaptionLine']">
        <xsl:text>Figure </xsl:text>
        <xsl:value-of select="$chapternumber" />
        <xsl:value-of select="$appendixnumber" />
        <xsl:text>-</xsl:text>
        <xsl:number level="any" count="w:p[w:pPr/w:pStyle/@w:val = 'CaptionLine']" />
        <xsl:text>: </xsl:text>
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:tbl">
        <xsl:apply-templates select="*" />
        <xsl:text>&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:tr[.//w:pStyle/@w:val = 'TableHeader']">
        <xsl:apply-templates select="*" />
        <xsl:text>|&#10;</xsl:text>
        <xsl:for-each select="w:tc">
            <xsl:text>|---</xsl:text>
        </xsl:for-each>
        <xsl:text>|&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:tr">
        <xsl:apply-templates select="*" />
        <xsl:text>|&#10;</xsl:text>
    </xsl:template>

    <xsl:template match="w:tc">
        <xsl:text>| </xsl:text>
        <xsl:apply-templates select=".//w:r" />
        <xsl:text> </xsl:text>
    </xsl:template>

    <xsl:template match="w:p[not(w:pPr)]" />

    <xsl:template match="w:p">
Unmatched: <xsl:value-of select="w:pPr/w:pStyle/@w:val" />
      <xsl:text>
      </xsl:text>


    </xsl:template>

    <!-- Character styles -->

    <xsl:template match="w:r[w:rPr/w:rStyle[@w:val = 'Literal' or @w:val = 'LiteralBold' or @w:val = 'LiteralCaption' or @w:val = 'LiteralBox']]">
        <xsl:choose>
            <xsl:when test="normalize-space(w:t) != ''">
                <xsl:if test="starts-with(w:t, ' ')">
                    <xsl:text> </xsl:text>
                </xsl:if>
                <xsl:if test="not(preceding-sibling::*[1][self::w:r]) or preceding-sibling::w:r[1][not(w:rPr/w:rStyle/@w:val = 'Literal') and not(w:rPr/w:rStyle/@w:val = 'LiteralBold') and not(w:rPr/w:rStyle/@w:val = 'LiteralCaption') and not(w:rPr/w:rStyle/@w:val = 'LiteralBox')]">
                    <xsl:text>`</xsl:text>
                </xsl:if>
                <xsl:value-of select="normalize-space(w:t)" />
                <xsl:if test="not(following-sibling::*[1][self::w:r]) or following-sibling::w:r[1][not(w:rPr/w:rStyle/@w:val = 'Literal') and not(w:rPr/w:rStyle/@w:val = 'LiteralBold') and not(w:rPr/w:rStyle/@w:val = 'LiteralCaption') and not(w:rPr/w:rStyle/@w:val = 'LiteralBox')] or following-sibling::w:r[1][w:rPr/w:rStyle/@w:val = 'Literal' and not(w:t)]">
                    <xsl:text>`</xsl:text>
                </xsl:if>
                <xsl:if test="substring(w:t, string-length(w:t)) = ' '">
                    <xsl:text> </xsl:text>
                </xsl:if>
            </xsl:when>
            <xsl:when test="normalize-space(w:t) != w:t and w:t != ''">
                <xsl:text> </xsl:text>
            </xsl:when>
        </xsl:choose>
    </xsl:template>

    <xsl:template match="w:r[w:rPr/w:rStyle[@w:val = 'EmphasisBold' or @w:val = 'Bold']]">
        <xsl:choose>
            <xsl:when test="normalize-space(w:t) != ''">
                <xsl:if test="starts-with(w:t, ' ')">
                    <xsl:text> </xsl:text>
                </xsl:if>
                <xsl:text>**</xsl:text>
                <xsl:value-of select="normalize-space(w:t)" />
                <xsl:text>**</xsl:text>
                <xsl:if test="substring(w:t, string-length(w:t)) = ' '">
                    <xsl:text> </xsl:text>
                </xsl:if>
            </xsl:when>
            <xsl:when test="normalize-space(w:t) != w:t and w:t != ''">
                <xsl:text> </xsl:text>
            </xsl:when>
        </xsl:choose>
    </xsl:template>

    <xsl:template match="w:r[w:rPr/w:rStyle[@w:val = 'EmphasisItalic' or @w:val = 'EmphasisItalicBox' or @w:val = 'EmphasisNote' or @w:val = 'EmphasisRevCaption' or @w:val = 'EmphasisRevItal' or @w:val = 'Italic' or @w:val = 'LinkURL']]">
        <xsl:choose>
            <xsl:when test="w:t and normalize-space(w:t) != ''">
                <xsl:if test="starts-with(w:t, ' ')">
                    <xsl:text> </xsl:text>
                </xsl:if>
                <xsl:if test="not(preceding-sibling::*[1][self::w:r]) or preceding-sibling::w:r[1][not(w:t) or not(w:rPr/w:rStyle/@w:val = 'EmphasisItalic') and not(w:rPr/w:rStyle/@w:val = 'EmphasisItalicBox') and not(w:rPr/w:rStyle/@w:val = 'EmphasisNote') and not(w:rPr/w:rStyle/@w:val = 'EmphasisRevCaption') and not(w:rPr/w:rStyle/@w:val = 'EmphasisRevItal') and not(w:rPr/w:rStyle/@w:val = 'Italic') and not(w:rPr/w:rStyle/@w:val = 'LinkURL')]">
                    <xsl:text>*</xsl:text>
                </xsl:if>

                <xsl:value-of select="normalize-space(w:t)" />

                <xsl:if test="not(following-sibling::*[1][self::w:r]) or following-sibling::w:r[1][not(w:t) or not(w:rPr/w:rStyle/@w:val = 'EmphasisItalic') and not(w:rPr/w:rStyle/@w:val = 'EmphasisItalicBox') and not(w:rPr/w:rStyle/@w:val = 'EmphasisNote') and not(w:rPr/w:rStyle/@w:val = 'EmphasisRevCaption') and not(w:rPr/w:rStyle/@w:val = 'EmphasisRevItal') and not(w:rPr/w:rStyle/@w:val = 'Italic') and not(w:rPr/w:rStyle/@w:val = 'LinkURL')]">
                    <xsl:text>*</xsl:text>
                </xsl:if>

                <xsl:if test="substring(w:t, string-length(w:t)) = ' '">
                    <xsl:text> </xsl:text>
                </xsl:if>
            </xsl:when>
            <xsl:when test="w:t">
                <xsl:text> </xsl:text>
            </xsl:when>
        </xsl:choose>
    </xsl:template>

    <xsl:template match="w:r[w:rPr/w:rStyle[@w:val = 'CodeAnnotation']]">
        <xsl:choose>
            <xsl:when test="normalize-space(w:t) != ''">
                <xsl:if test="starts-with(w:t, ' ')">
                    <xsl:text> </xsl:text>
                </xsl:if>
                <xsl:text>[</xsl:text>
                <xsl:value-of select="normalize-space(w:t)" />
                <xsl:text>]</xsl:text>
                <xsl:if test="substring(w:t, string-length(w:t)) = ' '">
                    <xsl:text> </xsl:text>
                </xsl:if>
            </xsl:when>
            <xsl:otherwise>
                <xsl:text> </xsl:text>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template match="w:r[w:rPr/w:rStyle[@w:val = 'Superscript']]">
        <xsl:if test="not(preceding-sibling::*[1][self::w:r]) or preceding-sibling::w:r[1][not(w:t) or not(w:rPr/w:rStyle/@w:val = 'Superscript')]">
            <xsl:text>&lt;sup></xsl:text>
        </xsl:if>
        <xsl:value-of select="w:t" />
        <xsl:if test="not(following-sibling::*[1][self::w:r]) or following-sibling::w:r[1][not(w:t) or not(w:rPr/w:rStyle/@w:val = 'Superscript')]">
            <xsl:text>&lt;/sup></xsl:text>
        </xsl:if>
    </xsl:template>

    <xsl:template match="w:r">
        <xsl:value-of select="w:t" />
    </xsl:template>
</xsl:stylesheet>
